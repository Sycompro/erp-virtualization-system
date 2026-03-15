package com.erpvirtualization.android.ui.screens

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.erpvirtualization.android.ui.components.*
import com.erpvirtualization.android.ui.theme.ERPColors

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SettingsScreen(
    onBack: () -> Unit
) {
    var selectedQuality by remember { mutableStateOf("Alta") }
    var selectedResolution by remember { mutableStateOf("1920x1080") }
    var selectedFrameRate by remember { mutableStateOf("60 FPS") }
    var hardwareAcceleration by remember { mutableStateOf(true) }
    var lowLatencyMode by remember { mutableStateOf(true) }
    var autoReconnect by remember { mutableStateOf(true) }
    
    Scaffold(
        topBar = {
            TopAppBar(
                title = { 
                    Text(
                        "Configuración de Visualización",
                        fontWeight = FontWeight.Bold
                    ) 
                },
                navigationIcon = {
                    IconButton(onClick = onBack) {
                        Icon(Icons.Default.ArrowBack, "Volver")
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = ERPColors.CorporateBlue,
                    titleContentColor = ERPColors.TextOnPrimary,
                    navigationIconContentColor = ERPColors.TextOnPrimary
                )
            )
        }
    ) { paddingValues ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(paddingValues)
                .verticalScroll(rememberScrollState())
                .padding(20.dp),
            verticalArrangement = Arrangement.spacedBy(20.dp)
        ) {
            // Sección: Calidad de Video
            Text(
                text = "Calidad de Video",
                style = MaterialTheme.typography.titleMedium,
                color = ERPColors.TextPrimary,
                fontWeight = FontWeight.Bold
            )
            
            ERPCard(style = ERPCardStyle.ELEVATED) {
                Column(verticalArrangement = Arrangement.spacedBy(16.dp)) {
                    SettingDropdown(
                        label = "Calidad",
                        value = selectedQuality,
                        options = listOf("Baja", "Media", "Alta", "Ultra"),
                        onValueChange = { selectedQuality = it },
                        icon = Icons.Default.HighQuality
                    )
                    
                    SettingDropdown(
                        label = "Resolución",
                        value = selectedResolution,
                        options = listOf("1280x720", "1920x1080", "2560x1440", "3840x2160"),
                        onValueChange = { selectedResolution = it },
                        icon = Icons.Default.AspectRatio
                    )
                    
                    SettingDropdown(
                        label = "Tasa de Frames",
                        value = selectedFrameRate,
                        options = listOf("30 FPS", "60 FPS", "120 FPS"),
                        onValueChange = { selectedFrameRate = it },
                        icon = Icons.Default.Speed
                    )
                }
            }
            
            // Sección: Rendimiento
            Text(
                text = "Rendimiento",
                style = MaterialTheme.typography.titleMedium,
                color = ERPColors.TextPrimary,
                fontWeight = FontWeight.Bold
            )
            
            ERPCard(style = ERPCardStyle.ELEVATED) {
                Column(verticalArrangement = Arrangement.spacedBy(16.dp)) {
                    SettingSwitch(
                        label = "Aceleración por Hardware",
                        description = "Usa GPU para decodificación de video",
                        checked = hardwareAcceleration,
                        onCheckedChange = { hardwareAcceleration = it },
                        icon = Icons.Default.Memory
                    )
                    
                    SettingSwitch(
                        label = "Modo Baja Latencia",
                        description = "Reduce el retraso en la transmisión",
                        checked = lowLatencyMode,
                        onCheckedChange = { lowLatencyMode = it },
                        icon = Icons.Default.Speed
                    )
                }
            }
            
            // Sección: Conexión
            Text(
                text = "Conexión",
                style = MaterialTheme.typography.titleMedium,
                color = ERPColors.TextPrimary,
                fontWeight = FontWeight.Bold
            )
            
            ERPCard(style = ERPCardStyle.ELEVATED) {
                Column(verticalArrangement = Arrangement.spacedBy(16.dp)) {
                    SettingSwitch(
                        label = "Reconexión Automática",
                        description = "Reconectar si se pierde la conexión",
                        checked = autoReconnect,
                        onCheckedChange = { autoReconnect = it },
                        icon = Icons.Default.Sync
                    )
                }
            }
            
            // Información del Sistema
            Text(
                text = "Información del Sistema",
                style = MaterialTheme.typography.titleMedium,
                color = ERPColors.TextPrimary,
                fontWeight = FontWeight.Bold
            )
            
            ERPCard(style = ERPCardStyle.INFO) {
                Column(verticalArrangement = Arrangement.spacedBy(12.dp)) {
                    InfoRow("Versión", "1.0.0")
                    InfoRow("Protocolo", "WebRTC")
                    InfoRow("Codec", "H.264/VP9")
                    InfoRow("Cifrado", "TLS 1.3 + mTLS")
                }
            }
            
            // Botones de acción
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                ERPButton(
                    text = "Restablecer",
                    onClick = { 
                        selectedQuality = "Alta"
                        selectedResolution = "1920x1080"
                        selectedFrameRate = "60 FPS"
                        hardwareAcceleration = true
                        lowLatencyMode = true
                        autoReconnect = true
                    },
                    style = ERPButtonStyle.OUTLINE,
                    size = ERPButtonSize.MEDIUM,
                    modifier = Modifier.weight(1f)
                )
                
                ERPButton(
                    text = "Guardar",
                    onClick = { /* Guardar configuración */ },
                    style = ERPButtonStyle.PRIMARY,
                    size = ERPButtonSize.MEDIUM,
                    icon = Icons.Default.Save,
                    modifier = Modifier.weight(1f)
                )
            }
        }
    }
}

@Composable
private fun SettingDropdown(
    label: String,
    value: String,
    options: List<String>,
    onValueChange: (String) -> Unit,
    icon: androidx.compose.ui.graphics.vector.ImageVector
) {
    var expanded by remember { mutableStateOf(false) }
    
    Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
        Row(
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            Icon(
                imageVector = icon,
                contentDescription = null,
                tint = ERPColors.CorporateBlue,
                modifier = Modifier.size(20.dp)
            )
            Text(
                text = label,
                style = MaterialTheme.typography.bodyMedium,
                color = ERPColors.TextPrimary,
                fontWeight = FontWeight.Medium
            )
        }
        
        ExposedDropdownMenuBox(
            expanded = expanded,
            onExpandedChange = { expanded = it }
        ) {
            OutlinedTextField(
                value = value,
                onValueChange = {},
                readOnly = true,
                trailingIcon = {
                    Icon(
                        imageVector = if (expanded) Icons.Default.ArrowDropUp else Icons.Default.ArrowDropDown,
                        contentDescription = null
                    )
                },
                modifier = Modifier
                    .fillMaxWidth()
                    .menuAnchor(),
                colors = OutlinedTextFieldDefaults.colors(
                    focusedBorderColor = ERPColors.CorporateBlue,
                    unfocusedBorderColor = ERPColors.ExecutiveGrayLight
                )
            )
            
            ExposedDropdownMenu(
                expanded = expanded,
                onDismissRequest = { expanded = false }
            ) {
                options.forEach { option ->
                    DropdownMenuItem(
                        text = { Text(option) },
                        onClick = {
                            onValueChange(option)
                            expanded = false
                        }
                    )
                }
            }
        }
    }
}

@Composable
private fun SettingSwitch(
    label: String,
    description: String,
    checked: Boolean,
    onCheckedChange: (Boolean) -> Unit,
    icon: androidx.compose.ui.graphics.vector.ImageVector
) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceBetween,
        verticalAlignment = Alignment.CenterVertically
    ) {
        Row(
            modifier = Modifier.weight(1f),
            horizontalArrangement = Arrangement.spacedBy(12.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Icon(
                imageVector = icon,
                contentDescription = null,
                tint = ERPColors.CorporateBlue,
                modifier = Modifier.size(24.dp)
            )
            
            Column {
                Text(
                    text = label,
                    style = MaterialTheme.typography.bodyMedium,
                    color = ERPColors.TextPrimary,
                    fontWeight = FontWeight.Medium
                )
                Text(
                    text = description,
                    style = MaterialTheme.typography.bodySmall,
                    color = ERPColors.TextSecondary
                )
            }
        }
        
        Switch(
            checked = checked,
            onCheckedChange = onCheckedChange,
            colors = SwitchDefaults.colors(
                checkedThumbColor = ERPColors.TextOnPrimary,
                checkedTrackColor = ERPColors.CorporateBlue,
                uncheckedThumbColor = ERPColors.ExecutiveGray,
                uncheckedTrackColor = ERPColors.ExecutiveGrayLight
            )
        )
    }
}

@Composable
private fun InfoRow(label: String, value: String) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceBetween
    ) {
        Text(
            text = label,
            style = MaterialTheme.typography.bodyMedium,
            color = ERPColors.TextSecondary
        )
        Text(
            text = value,
            style = MaterialTheme.typography.bodyMedium,
            color = ERPColors.TextPrimary,
            fontWeight = FontWeight.Medium
        )
    }
}
