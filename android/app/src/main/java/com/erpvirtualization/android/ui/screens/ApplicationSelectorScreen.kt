package com.erpvirtualization.android.ui.screens

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.background
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.erpvirtualization.android.ui.components.*
import com.erpvirtualization.android.ui.theme.ERPColors

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ApplicationSelectorScreen(
    onApplicationSelected: (ApplicationInfo) -> Unit,
    onBackPressed: () -> Unit
) {
    var selectedCategory by remember { mutableStateOf("Todos") }
    var viewMode by remember { mutableStateOf(ViewMode.GRID) }
    
    // Datos de ejemplo - en producción vendrían del servidor
    val applications = remember { getSampleApplications() }
    val categories = remember { 
        listOf("Todos") + applications.map { it.category }.distinct().sorted()
    }
    
    val filteredApps = if (selectedCategory == "Todos") {
        applications
    } else {
        applications.filter { it.category == selectedCategory }
    }
    
    Column(
        modifier = Modifier.fillMaxSize()
    ) {
        // Top Bar
        ERPTopBar(
            title = "Seleccionar Aplicación",
            subtitle = "${filteredApps.size} aplicaciones disponibles",
            navigationIcon = Icons.Default.ArrowBack,
            onNavigationClick = onBackPressed,
            actions = {
                ERPActionButton(
                    icon = if (viewMode == ViewMode.GRID) Icons.Default.ViewList else Icons.Default.GridView,
                    contentDescription = "Cambiar vista",
                    onClick = { 
                        viewMode = if (viewMode == ViewMode.GRID) ViewMode.LIST else ViewMode.GRID
                    }
                )
            }
        )
        
        // Filtros de categoría
        LazyRow(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            items(categories) { category ->
                FilterChip(
                    onClick = { selectedCategory = category },
                    label = { Text(category) },
                    selected = selectedCategory == category,
                    colors = FilterChipDefaults.filterChipColors(
                        selectedContainerColor = ERPColors.CorporateBlue,
                        selectedLabelColor = ERPColors.TextOnPrimary
                    )
                )
            }
        }
        
        // Lista/Grid de aplicaciones
        when (viewMode) {
            ViewMode.GRID -> {
                LazyVerticalGrid(
                    columns = GridCells.Fixed(2),
                    modifier = Modifier.fillMaxSize(),
                    contentPadding = PaddingValues(16.dp),
                    horizontalArrangement = Arrangement.spacedBy(12.dp),
                    verticalArrangement = Arrangement.spacedBy(12.dp)
                ) {
                    items(filteredApps) { app ->
                        ApplicationGridCard(
                            application = app,
                            onClick = { onApplicationSelected(app) }
                        )
                    }
                }
            }
            ViewMode.LIST -> {
                LazyColumn(
                    modifier = Modifier.fillMaxSize(),
                    contentPadding = PaddingValues(16.dp),
                    verticalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    items(filteredApps) { app ->
                        ApplicationListCard(
                            application = app,
                            onClick = { onApplicationSelected(app) }
                        )
                    }
                }
            }
        }
    }
}

@Composable
fun ApplicationGridCard(
    application: ApplicationInfo,
    onClick: () -> Unit
) {
    ERPCard(
        style = ERPCardStyle.ELEVATED,
        onClick = onClick,
        modifier = Modifier
            .fillMaxWidth()
            .height(160.dp)
    ) {
        Column(
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            // Icono de la aplicación
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
                    imageVector = getApplicationIcon(application.id),
                    contentDescription = null,
                    modifier = Modifier.size(28.dp),
                    tint = ERPColors.CorporateBlue
                )
            }
            
            // Nombre de la aplicación
            Text(
                text = application.name,
                style = MaterialTheme.typography.titleSmall,
                color = ERPColors.TextPrimary,
                fontWeight = FontWeight.SemiBold,
                maxLines = 2
            )
            
            // Categoría
            Text(
                text = application.category,
                style = MaterialTheme.typography.labelSmall,
                color = ERPColors.TextSecondary
            )
            
            // Indicador de protocolo
            Box(
                modifier = Modifier
                    .background(
                        color = if (application.displayProtocol == "RDP") 
                            ERPColors.SoftMint else ERPColors.SoftSky,
                        shape = androidx.compose.foundation.shape.RoundedCornerShape(8.dp)
                    )
                    .padding(horizontal = 8.dp, vertical = 2.dp)
            ) {
                Text(
                    text = application.displayProtocol,
                    style = MaterialTheme.typography.labelSmall,
                    color = if (application.displayProtocol == "RDP") 
                        ERPColors.EnterpriseGreen else ERPColors.InfoBlue,
                    fontWeight = FontWeight.Medium
                )
            }
        }
    }
}

@Composable
fun ApplicationListCard(
    application: ApplicationInfo,
    onClick: () -> Unit
) {
    ERPCard(
        style = ERPCardStyle.FLAT,
        onClick = onClick,
        modifier = Modifier.fillMaxWidth()
    ) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(16.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            // Icono
            Box(
                modifier = Modifier
                    .size(56.dp)
                    .background(
                        color = ERPColors.SoftLavender,
                        shape = androidx.compose.foundation.shape.RoundedCornerShape(12.dp)
                    ),
                contentAlignment = Alignment.Center
            ) {
                Icon(
                    imageVector = getApplicationIcon(application.id),
                    contentDescription = null,
                    modifier = Modifier.size(32.dp),
                    tint = ERPColors.CorporateBlue
                )
            }
            
            // Información
            Column(
                modifier = Modifier.weight(1f),
                verticalArrangement = Arrangement.spacedBy(4.dp)
            ) {
                Text(
                    text = application.name,
                    style = MaterialTheme.typography.titleMedium,
                    color = ERPColors.TextPrimary,
                    fontWeight = FontWeight.SemiBold
                )
                
                Text(
                    text = application.description,
                    style = MaterialTheme.typography.bodySmall,
                    color = ERPColors.TextSecondary,
                    maxLines = 2
                )
                
                Row(
                    horizontalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    // Categoría
                    Box(
                        modifier = Modifier
                            .background(
                                color = ERPColors.SoftPeach,
                                shape = androidx.compose.foundation.shape.RoundedCornerShape(6.dp)
                            )
                            .padding(horizontal = 6.dp, vertical = 2.dp)
                    ) {
                        Text(
                            text = application.category,
                            style = MaterialTheme.typography.labelSmall,
                            color = ERPColors.AccentGold
                        )
                    }
                    
                    // Protocolo
                    Box(
                        modifier = Modifier
                            .background(
                                color = if (application.displayProtocol == "RDP") 
                                    ERPColors.SoftMint else ERPColors.SoftSky,
                                shape = androidx.compose.foundation.shape.RoundedCornerShape(6.dp)
                            )
                            .padding(horizontal = 6.dp, vertical = 2.dp)
                    ) {
                        Text(
                            text = application.displayProtocol,
                            style = MaterialTheme.typography.labelSmall,
                            color = if (application.displayProtocol == "RDP") 
                                ERPColors.EnterpriseGreen else ERPColors.InfoBlue
                        )
                    }
                }
            }
            
            // Flecha
            Icon(
                imageVector = Icons.Default.ChevronRight,
                contentDescription = null,
                tint = ERPColors.TextTertiary
            )
        }
    }
}

@Composable
private fun getApplicationIcon(appId: String): ImageVector {
    return when (appId) {
        "sap", "oracle", "dynamics" -> Icons.Default.Business
        "office", "libreoffice" -> Icons.Default.Description
        "autocad" -> Icons.Default.Architecture
        "adobe", "gimp" -> Icons.Default.Palette
        "blender" -> Icons.Default.ViewInAr
        "visualstudio" -> Icons.Default.Code
        "windows-desktop", "ubuntu-desktop" -> Icons.Default.Computer
        else -> Icons.Default.Apps
    }
}

private fun getSampleApplications(): List<ApplicationInfo> {
    return listOf(
        ApplicationInfo(
            id = "sap",
            name = "SAP GUI",
            category = "ERP Systems",
            description = "Sistema ERP empresarial SAP con interfaz completa",
            displayProtocol = "VNC",
            features = listOf("Streaming HD", "Touch optimizado", "Clipboard sync")
        ),
        ApplicationInfo(
            id = "office",
            name = "Microsoft Office",
            category = "Office Suite",
            description = "Word, Excel, PowerPoint, Outlook completos",
            displayProtocol = "RDP",
            features = listOf("Streaming HD", "Impresión remota", "Compartir archivos")
        ),
        ApplicationInfo(
            id = "autocad",
            name = "AutoCAD",
            category = "Design & CAD",
            description = "Diseño CAD profesional 2D y 3D",
            displayProtocol = "RDP",
            features = listOf("Aceleración GPU", "Precisión de color", "Touch optimizado")
        ),
        ApplicationInfo(
            id = "adobe",
            name = "Adobe Creative Suite",
            category = "Design & CAD",
            description = "Photoshop, Illustrator, InDesign, Premiere",
            displayProtocol = "RDP",
            features = listOf("Aceleración GPU", "Precisión de color", "Streaming 4K")
        ),
        ApplicationInfo(
            id = "visualstudio",
            name = "Visual Studio",
            category = "Development",
            description = "IDE completo para desarrollo .NET",
            displayProtocol = "RDP",
            features = listOf("Streaming HD", "Clipboard sync", "Debugging remoto")
        ),
        ApplicationInfo(
            id = "windows-desktop",
            name = "Windows Desktop",
            category = "Full Desktop",
            description = "Escritorio Windows completo con todas las aplicaciones",
            displayProtocol = "RDP",
            features = listOf("Escritorio completo", "Múltiples aplicaciones", "Gestión de archivos")
        )
    )
}

data class ApplicationInfo(
    val id: String,
    val name: String,
    val category: String,
    val description: String,
    val displayProtocol: String,
    val features: List<String>
)

enum class ViewMode {
    GRID, LIST
}