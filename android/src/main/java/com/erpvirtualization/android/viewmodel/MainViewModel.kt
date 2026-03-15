package com.erpvirtualization.android.viewmodel

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.erpvirtualization.android.data.repository.AuthRepository
import com.erpvirtualization.android.data.repository.StreamingRepository
import com.erpvirtualization.android.data.repository.TouchAction
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import timber.log.Timber
import javax.inject.Inject

@HiltViewModel
class MainViewModel @Inject constructor(
    private val authRepository: AuthRepository,
    private val streamingRepository: StreamingRepository
) : ViewModel() {

    private val _uiState = MutableStateFlow(MainUiState())
    val uiState: StateFlow<MainUiState> = _uiState.asStateFlow()

    init {
        // Observar estado de conexión de streaming
        viewModelScope.launch {
            streamingRepository.connectionState.collect { connectionState ->
                _uiState.value = _uiState.value.copy(
                    connectionState = when (connectionState) {
                        com.erpvirtualization.android.data.repository.StreamingConnectionState.DISCONNECTED -> ConnectionState.DISCONNECTED
                        com.erpvirtualization.android.data.repository.StreamingConnectionState.CONNECTING -> ConnectionState.CONNECTING
                        com.erpvirtualization.android.data.repository.StreamingConnectionState.CONNECTED -> ConnectionState.CONNECTED
                        com.erpvirtualization.android.data.repository.StreamingConnectionState.ERROR -> ConnectionState.ERROR
                    }
                )
            }
        }
        
        // Observar estadísticas de streaming
        viewModelScope.launch {
            streamingRepository.streamingStats.collect { stats ->
                _uiState.value = _uiState.value.copy(
                    streamingStats = stats
                )
            }
        }
    }

    fun onBiometricAuthSuccess() {
        Timber.d("🔐 Iniciando proceso de autenticación")
        
        viewModelScope.launch {
            try {
                _uiState.value = _uiState.value.copy(
                    connectionState = ConnectionState.AUTHENTICATING,
                    errorMessage = null
                )
                
                // Autenticar con el servidor
                val authResult = authRepository.authenticateWithBiometrics()
                
                if (authResult.isSuccess) {
                    Timber.d("✅ Autenticación exitosa")
                    connectToERP()
                } else {
                    throw Exception("Fallo en autenticación: ${authResult.exceptionOrNull()?.message}")
                }
                
            } catch (e: Exception) {
                Timber.e(e, "❌ Error en autenticación")
                _uiState.value = _uiState.value.copy(
                    connectionState = ConnectionState.ERROR,
                    errorMessage = e.message ?: "Error desconocido en autenticación"
                )
            }
        }
    }
    
    private suspend fun connectToERP() {
        try {
            _uiState.value = _uiState.value.copy(
                connectionState = ConnectionState.CONNECTING
            )
            
            Timber.d("🔗 Estableciendo conexión WebRTC")
            
            // Conectar al servidor de streaming
            val connectionResult = streamingRepository.connectToServer()
            
            if (connectionResult.isSuccess) {
                Timber.d("✅ Conexión WebRTC establecida")
                
                // Iniciar stream del ERP
                startERPStream()
                
            } else {
                throw Exception("Fallo en conexión: ${connectionResult.exceptionOrNull()?.message}")
            }
            
        } catch (e: Exception) {
            Timber.e(e, "❌ Error estableciendo conexión")
            _uiState.value = _uiState.value.copy(
                connectionState = ConnectionState.ERROR,
                errorMessage = e.message ?: "Error desconocido en conexión"
            )
        }
    }
    
    private suspend fun startERPStream() {
        try {
            Timber.d("🖥️ Iniciando stream ERP")
            
            // Solicitar inicio de container ERP
            val containerResult = streamingRepository.startERPContainer("sap") // Ejemplo con SAP
            
            if (containerResult.isSuccess) {
                val containerId = containerResult.getOrNull()
                Timber.d("📦 Container ERP iniciado: $containerId")
                
                // Iniciar recepción de video
                streamingRepository.startVideoStreaming { videoFrame ->
                    handleVideoFrame(videoFrame)
                }
                
                _uiState.value = _uiState.value.copy(
                    connectionState = ConnectionState.CONNECTED,
                    containerInfo = ContainerInfo(
                        id = containerId ?: "",
                        type = "sap",
                        status = "running"
                    )
                )
                
            } else {
                throw Exception("Fallo iniciando container: ${containerResult.exceptionOrNull()?.message}")
            }
            
        } catch (e: Exception) {
            Timber.e(e, "❌ Error iniciando stream ERP")
            _uiState.value = _uiState.value.copy(
                connectionState = ConnectionState.ERROR,
                errorMessage = e.message ?: "Error iniciando stream ERP"
            )
        }
    }
    
    private fun handleVideoFrame(videoFrame: ByteArray) {
        // Procesar frame de video recibido del stream WebRTC
        Timber.v("📹 Frame de video recibido: ${videoFrame.size} bytes")
        
        // Aquí actualizarías la UI con el nuevo frame
        // En una implementación real, esto se haría a través de un Surface o TextureView
    }
    
    fun disconnect() {
        Timber.d("🔌 Desconectando del ERP")
        
        viewModelScope.launch {
            try {
                // Detener streaming
                streamingRepository.stopVideoStreaming()
                
                // Detener container ERP si existe
                _uiState.value.containerInfo?.let { containerInfo ->
                    streamingRepository.stopERPContainer(containerInfo.id)
                }
                
                // Desconectar del servidor
                streamingRepository.disconnect()
                
                // Limpiar autenticación
                authRepository.logout()
                
                _uiState.value = MainUiState() // Reset a estado inicial
                
                Timber.d("✅ Desconexión completada")
                
            } catch (e: Exception) {
                Timber.e(e, "❌ Error durante desconexión")
                _uiState.value = _uiState.value.copy(
                    connectionState = ConnectionState.ERROR,
                    errorMessage = "Error durante desconexión: ${e.message}"
                )
            }
        }
    }
    
    fun onBiometricAuthError(error: String) {
        Timber.e("❌ Error de autenticación biométrica: $error")
        _uiState.value = _uiState.value.copy(
            connectionState = ConnectionState.ERROR,
            errorMessage = "Error de autenticación: $error"
        )
    }
    
    fun sendTouchEvent(x: Float, y: Float, action: TouchAction) {
        viewModelScope.launch {
            try {
                // Enviar evento touch al servidor para interacción con ERP
                streamingRepository.sendTouchEvent(x, y, action)
                Timber.v("👆 Evento touch enviado: ($x, $y) - $action")
                
            } catch (e: Exception) {
                Timber.e(e, "❌ Error enviando evento touch")
            }
        }
    }
    
    fun sendKeyboardInput(text: String) {
        viewModelScope.launch {
            try {
                // Enviar entrada de teclado al servidor
                streamingRepository.sendKeyboardInput(text)
                Timber.v("⌨️ Entrada de teclado enviada: $text")
                
            } catch (e: Exception) {
                Timber.e(e, "❌ Error enviando entrada de teclado")
            }
        }
    }
    
    override fun onCleared() {
        super.onCleared()
        Timber.d("🧹 Limpiando ViewModel")
        
        // La limpieza se maneja automáticamente por los repositorios
        // y el sistema de inyección de dependencias
    }
}

data class MainUiState(
    val connectionState: ConnectionState = ConnectionState.DISCONNECTED,
    val errorMessage: String? = null,
    val containerInfo: ContainerInfo? = null,
    val streamingStats: com.erpvirtualization.android.data.repository.StreamingStats? = null
)

enum class ConnectionState {
    DISCONNECTED,
    AUTHENTICATING,
    CONNECTING,
    CONNECTED,
    ERROR
}

data class ContainerInfo(
    val id: String,
    val type: String,
    val status: String
)

data class ContainerInfo(
    val id: String,
    val type: String,
    val status: String
)