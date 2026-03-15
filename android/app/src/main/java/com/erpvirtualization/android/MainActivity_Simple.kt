package com.erpvirtualization.android

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.erpvirtualization.android.ui.theme.ERPVirtualizationTheme
import com.erpvirtualization.android.ui.theme.ERPColors
import com.erpvirtualization.android.ui.components.*
import com.erpvirtualization.android.ui.screens.SettingsScreen
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController

// Versión simplificada sin Hilt para debugging
class MainActivity_Simple : ComponentActivity() {
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        setContent {
            ERPVirtualizationTheme {
                val navController = rememberNavController()
                
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    NavHost(
                        navController = navController,
                        startDestination = "main"
                    ) {
                        composable("main") {
                            SimpleMainScreen(
                                onNavigateToSettings = { navController.navigate("settings") }
                            )
                        }
                        composable("settings") {
                            SettingsScreen(
                                onBack = { navController.popBackStack() }
                            )
                        }
                    }
                }
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SimpleMainScreen(
    onNavigateToSettings: () -> Unit = {}
) {
    var connectionState by remember { mutableStateOf("DISCONNECTED") }
    
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
                        IconButton(onClick = onNavigateToSettings) {
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
                
                Spacer(modifier = Modifier.height(24.dp))
                
                // Título
                Text(
                    text = "Bienvenido a ERP Virtualización",
                    style = MaterialTheme.typography.headlineMedium,
                    color = ERPColors.TextPrimary,
                    fontWeight = FontWeight.Bold
                )
                
                Spacer(modifier = Modifier.height(8.dp))
                
                Text(
                    text = "Versión de prueba - Sin autenticación",
                    style = MaterialTheme.typography.bodyLarge,
                    color = ERPColors.TextSecondary
                )
                
                Spacer(modifier = Modifier.height(32.dp))
                
                // Estado
                ERPCard(
                    style = ERPCardStyle.INFO,
                    modifier = Modifier.fillMaxWidth()
                ) {
                    Column(
                        horizontalAlignment = Alignment.CenterHorizontally,
                        verticalArrangement = Arrangement.spacedBy(16.dp)
                    ) {
                        Icon(
                            imageVector = Icons.Default.CheckCircle,
                            contentDescription = null,
                            modifier = Modifier.size(48.dp),
                            tint = ERPColors.SuccessGreen
                        )
                        
                        Text(
                            text = "App funcionando correctamente",
                            style = MaterialTheme.typography.titleMedium,
                            color = ERPColors.TextPrimary,
                            fontWeight = FontWeight.SemiBold
                        )
                        
                        Text(
                            text = "Estado: $connectionState",
                            style = MaterialTheme.typography.bodyMedium,
                            color = ERPColors.TextSecondary
                        )
                    }
                }
                
                Spacer(modifier = Modifier.height(24.dp))
                
                // Botón de prueba
                ERPButton(
                    text = "Ir a Configuración",
                    onClick = onNavigateToSettings,
                    style = ERPButtonStyle.PRIMARY,
                    size = ERPButtonSize.LARGE,
                    icon = Icons.Default.Settings,
                    modifier = Modifier.fillMaxWidth()
                )
                
                Spacer(modifier = Modifier.height(16.dp))
                
                ERPButton(
                    text = "Simular Conexión",
                    onClick = { 
                        connectionState = if (connectionState == "DISCONNECTED") "CONNECTED" else "DISCONNECTED"
                    },
                    style = ERPButtonStyle.OUTLINE,
                    size = ERPButtonSize.LARGE,
                    icon = Icons.Default.Power,
                    modifier = Modifier.fillMaxWidth()
                )
            }
        }
    }
}
