package com.erpvirtualization.android

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricPrompt
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity
import androidx.hilt.navigation.compose.hiltViewModel
import com.erpvirtualization.android.ui.theme.ERPVirtualizationTheme
import com.erpvirtualization.android.ui.theme.ERPColors
import com.erpvirtualization.android.ui.components.*
import com.erpvirtualization.android.viewmodel.*
import dagger.hilt.android.AndroidEntryPoint
import timber.log.Timber

@AndroidEntryPoint
class MainActivity : ComponentActivity() {
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // Inicializar logging
        if (BuildConfig.DEBUG) {
            Timber.plant(Timber.DebugTree())
        }
        
        enableEdgeToEdge()
        
        setContent {
            ERPVirtualizationTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    MainScreen()
                }
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MainScreen(
    viewModel: MainViewModel = hiltViewModel()
) {
    val context = LocalContext.current
    val uiState by viewModel.uiState.collectAsState()
    
    var showBiometricPrompt by remember { mutableStateOf(false) }
    
    // Configurar autenticación biométrica
    val biometricPrompt = remember {
        BiometricPrompt(
            context as FragmentActivity,
            ContextCompat.getMainExecutor(context),
            object : BiometricPrompt.AuthenticationCallback() {
                override fun onAuthenticationSucceeded(result: BiometricPrompt.AuthenticationResult) {
                    super.onAuthenticationSucceeded(result)
                    Timber.d("🔐 Autenticación biométrica exitosa")
                    viewModel.onBiometricAuthSuccess()
                }
                
                override fun onAuthenticationError(errorCode: Int, errString: CharSequence) {
                    super.onAuthenticationError(errorCode, errString)
                    Timber.e("❌ Error de autenticación: $errString")
                    viewModel.onBiometricAuthError(errString.toString())
                }
            }
        )
    }
    
    val promptInfo = remember {
        BiometricPrompt.PromptInfo.Builder()
            .setTitle("Acceso Seguro ERP")
            .setSubtitle("Usa tu huella dactilar o reconocimiento facial")
            .setNegativeButtonText("Cancelar")
            .setAllowedAuthenticators(
                BiometricManager.Authenticators.BIOMETRIC_STRONG or
                BiometricManager.Authenticators.DEVICE_CREDENTIAL
            )
            .build()
    }
    
    LaunchedEffect(showBiometricPrompt) {
        if (showBiometricPrompt) {
            biometricPrompt.authenticate(promptInfo)
            showBiometricPrompt = false
        }
    }
    
    // Fondo con gradiente empresarial
    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(
                brush = Brush.verticalGradient(
                    colors = listOf(
                        ERPColors.SoftLavender,
                        ERPColors.SurfacePrimary,
                        ERPColors.SoftSky
                    )
                )
            )
    ) {
        Scaffold(
            containerColor = androidx.compose.ui.graphics.Color.Transparent,
            topBar = {
                TopAppBar(
                    title = { 
                        Text(
                            "ERP Virtualización",
                            style = MaterialTheme.typography.titleLarge,
                            fontWeight = FontWeight.Bold,
                            color = ERPColors.TextOnPrimary
                        ) 
                    },
                    colors = TopAppBarDefaults.topAppBarColors(
                        containerColor = ERPColors.CorporateBlue
                    ),
                    actions = {
                        IconButton(onClick = { /* Settings */ }) {
                            Icon(
                                Icons.Default.Settings,
                                contentDescription = "Configuración",
                                tint = ERPColors.TextOnPrimary
                            )
                        }
                    }
                )
            }
        ) { paddingValues ->
            Column(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(paddingValues)
                    .padding(24.dp),
                horizontalAlignment = Alignment.CenterHorizontally,
                verticalArrangement = Arrangement.Center
            ) {
                when (uiState.connectionState) {
                    ConnectionState.DISCONNECTED -> {
                        DisconnectedScreen(
                            onConnect = { showBiometricPrompt = true }
                        )
                    }
                    ConnectionState.AUTHENTICATING -> {
                        AuthenticatingScreen()
                    }
                    ConnectionState.CONNECTING -> {
                        ConnectingScreen()
                    }
                    ConnectionState.CONNECTED -> {
                        ConnectedScreen(
                            containerInfo = uiState.containerInfo,
                            onDisconnect = { viewModel.disconnect() }
                        )
                    }
                    ConnectionState.ERROR -> {
                        ErrorScreen(
                            error = uiState.errorMessage,
                            onRetry = { showBiometricPrompt = true }
                        )
                    }
                }
            }
        }
    }
}

@Composable
fun DisconnectedScreen(onConnect: () -> Unit) {
    Column(
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(24.dp)
    ) {
        // Logo/Icono principal
        ERPCard(
            style = ERPCardStyle.GRADIENT,
            modifier = Modifier.size(120.dp)
        ) {
            Box(
                modifier = Modifier.fillMaxSize(),
                contentAlignment = Alignment.Center
            ) {
                Icon(
                    imageVector = Icons.Default.Computer,
                    contentDescription = null,
                    modifier = Modifier.size(64.dp),
                    tint = ERPColors.CorporateBlue
                )
            }
        }
        
        // Título y descripción
        Column(
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            Text(
                text = "Conectar al ERP",
                style = MaterialTheme.typography.headlineMedium,
                color = ERPColors.TextPrimary,
                fontWeight = FontWeight.Bold
            )
            
            Text(
                text = "Accede a tu sistema ERP de forma segura\ncon autenticación biométrica avanzada",
                style = MaterialTheme.typography.bodyLarge,
                color = ERPColors.TextSecondary,
                textAlign = androidx.compose.ui.text.style.TextAlign.Center
            )
        }
        
        // Características de seguridad
        ERPCard(
            style = ERPCardStyle.FLAT,
            modifier = Modifier.fillMaxWidth()
        ) {
            Column(
                verticalArrangement = Arrangement.spacedBy(16.dp)
            ) {
                SecurityFeatureItem(
                    icon = Icons.Default.Fingerprint,
                    title = "Autenticación Biométrica",
                    description = "Huella dactilar y reconocimiento facial"
                )
                
                SecurityFeatureItem(
                    icon = Icons.Default.Security,
                    title = "Conexión Cifrada",
                    description = "Protocolo TLS 1.3 y mTLS"
                )
                
                SecurityFeatureItem(
                    icon = Icons.Default.Speed,
                    title = "Baja Latencia",
                    description = "Streaming WebRTC optimizado"
                )
            }
        }
        
        // Botón de conexión
        ERPButton(
            text = "Conectar con Biometría",
            onClick = onConnect,
            style = ERPButtonStyle.GRADIENT,
            size = ERPButtonSize.LARGE,
            icon = Icons.Default.Fingerprint,
            modifier = Modifier.fillMaxWidth()
        )
    }
}

@Composable
private fun SecurityFeatureItem(
    icon: androidx.compose.ui.graphics.vector.ImageVector,
    title: String,
    description: String
) {
    Row(
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        Box(
            modifier = Modifier
                .size(48.dp)
                .background(
                    color = ERPColors.SoftLavender,
                    shape = androidx.compose.foundation.shape.RoundedCornerShape(12.dp)
                ),
            contentAlignment = Alignment.Center
        ) {
            Icon(
                imageVector = icon,
                contentDescription = null,
                modifier = Modifier.size(24.dp),
                tint = ERPColors.CorporateBlue
            )
        }
        
        Column {
            Text(
                text = title,
                style = MaterialTheme.typography.titleSmall,
                color = ERPColors.TextPrimary,
                fontWeight = FontWeight.SemiBold
            )
            Text(
                text = description,
                style = MaterialTheme.typography.bodySmall,
                color = ERPColors.TextSecondary
            )
        }
    }
}

@Composable
fun AuthenticatingScreen() {
    ERPCard(
        style = ERPCardStyle.ELEVATED,
        modifier = Modifier.fillMaxWidth()
    ) {
        Column(
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.spacedBy(20.dp)
        ) {
            CircularProgressIndicator(
                modifier = Modifier.size(48.dp),
                color = ERPColors.CorporateBlue,
                strokeWidth = 4.dp
            )
            
            Text(
                text = "Autenticando...",
                style = MaterialTheme.typography.titleMedium,
                color = ERPColors.TextPrimary,
                fontWeight = FontWeight.SemiBold
            )
            
            Text(
                text = "Verificando tu identidad de forma segura",
                style = MaterialTheme.typography.bodyMedium,
                color = ERPColors.TextSecondary
            )
        }
    }
}

@Composable
fun ConnectingScreen() {
    ERPCard(
        style = ERPCardStyle.INFO,
        modifier = Modifier.fillMaxWidth()
    ) {
        Column(
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.spacedBy(20.dp)
        ) {
            CircularProgressIndicator(
                modifier = Modifier.size(48.dp),
                color = ERPColors.InfoBlue,
                strokeWidth = 4.dp
            )
            
            Text(
                text = "Estableciendo conexión segura...",
                style = MaterialTheme.typography.titleMedium,
                color = ERPColors.TextPrimary,
                fontWeight = FontWeight.SemiBold
            )
            
            Text(
                text = "Iniciando túnel cifrado WebRTC",
                style = MaterialTheme.typography.bodyMedium,
                color = ERPColors.TextSecondary
            )
        }
    }
}

@Composable
fun ConnectedScreen(
    containerInfo: ContainerInfo?,
    onDisconnect: () -> Unit
) {
    Column(
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(24.dp)
    ) {
        // Estado de conexión
        ERPStatusCard(
            title = "Estado de Conexión",
            status = "Conectado",
            statusColor = ERPColors.SuccessGreen,
            description = "ERP ${containerInfo?.type?.uppercase() ?: "Sistema"} activo",
            modifier = Modifier.fillMaxWidth()
        )
        
        // Información del container
        if (containerInfo != null) {
            ERPInfoCard(
                title = "Sistema ERP",
                subtitle = "Container ID: ${containerInfo.id.take(12)}...",
                value = containerInfo.type.uppercase(),
                icon = Icons.Default.Computer,
                modifier = Modifier.fillMaxWidth()
            )
        }
        
        // Área de streaming
        ERPCard(
            style = ERPCardStyle.ELEVATED,
            modifier = Modifier
                .fillMaxWidth()
                .height(400.dp)
        ) {
            Box(
                modifier = Modifier.fillMaxSize(),
                contentAlignment = Alignment.Center
            ) {
                Column(
                    horizontalAlignment = Alignment.CenterHorizontally,
                    verticalArrangement = Arrangement.spacedBy(16.dp)
                ) {
                    Icon(
                        imageVector = Icons.Default.Monitor,
                        contentDescription = null,
                        modifier = Modifier.size(64.dp),
                        tint = ERPColors.CorporateBlue
                    )
                    
                    Text(
                        text = "🖥️ Stream ERP Activo",
                        style = MaterialTheme.typography.titleLarge,
                        color = ERPColors.TextPrimary,
                        fontWeight = FontWeight.SemiBold
                    )
                    
                    Text(
                        text = "Interactúa con tu ERP como una app nativa",
                        style = MaterialTheme.typography.bodyMedium,
                        color = ERPColors.TextSecondary
                    )
                }
            }
        }
        
        // Controles
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            ERPButton(
                text = "Pantalla Completa",
                onClick = { /* Implementar */ },
                style = ERPButtonStyle.OUTLINE,
                size = ERPButtonSize.MEDIUM,
                icon = Icons.Default.Fullscreen,
                modifier = Modifier.weight(1f)
            )
            
            ERPButton(
                text = "Desconectar",
                onClick = onDisconnect,
                style = ERPButtonStyle.ERROR,
                size = ERPButtonSize.MEDIUM,
                icon = Icons.Default.PowerSettingsNew,
                modifier = Modifier.weight(1f)
            )
        }
    }
}

@Composable
fun ErrorScreen(error: String?, onRetry: () -> Unit) {
    Column(
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(24.dp)
    ) {
        ERPCard(
            style = ERPCardStyle.ERROR,
            modifier = Modifier.fillMaxWidth()
        ) {
            Column(
                horizontalAlignment = Alignment.CenterHorizontally,
                verticalArrangement = Arrangement.spacedBy(16.dp)
            ) {
                Icon(
                    imageVector = Icons.Default.Error,
                    contentDescription = null,
                    modifier = Modifier.size(64.dp),
                    tint = ERPColors.ErrorRed
                )
                
                Text(
                    text = "Error de Conexión",
                    style = MaterialTheme.typography.headlineSmall,
                    color = ERPColors.TextPrimary,
                    fontWeight = FontWeight.Bold
                )
                
                error?.let {
                    Text(
                        text = it,
                        style = MaterialTheme.typography.bodyMedium,
                        color = ERPColors.TextSecondary,
                        textAlign = androidx.compose.ui.text.style.TextAlign.Center
                    )
                }
            }
        }
        
        ERPButton(
            text = "Reintentar Conexión",
            onClick = onRetry,
            style = ERPButtonStyle.PRIMARY,
            size = ERPButtonSize.LARGE,
            icon = Icons.Default.Refresh,
            modifier = Modifier.fillMaxWidth()
        )
    }
}