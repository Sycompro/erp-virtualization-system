package com.erpvirtualization.android.ui.theme

import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Shapes
import androidx.compose.ui.unit.dp

val ERPShapes = Shapes(
    // Formas Pequeñas - Botones, chips, badges
    small = RoundedCornerShape(8.dp),
    
    // Formas Medianas - Cards, dialogs
    medium = RoundedCornerShape(16.dp),
    
    // Formas Grandes - Bottom sheets, modals
    large = RoundedCornerShape(24.dp)
)

// Formas Personalizadas para Componentes Específicos
object ERPCustomShapes {
    // Botones Principales
    val ButtonPrimary = RoundedCornerShape(12.dp)
    val ButtonSecondary = RoundedCornerShape(8.dp)
    val ButtonFab = RoundedCornerShape(16.dp)
    
    // Cards y Contenedores
    val CardElevated = RoundedCornerShape(20.dp)
    val CardFlat = RoundedCornerShape(12.dp)
    val CardCompact = RoundedCornerShape(8.dp)
    
    // Inputs y Campos
    val InputField = RoundedCornerShape(12.dp)
    val SearchBar = RoundedCornerShape(24.dp)
    
    // Navegación
    val BottomNavigation = RoundedCornerShape(
        topStart = 24.dp,
        topEnd = 24.dp,
        bottomStart = 0.dp,
        bottomEnd = 0.dp
    )
    val TopAppBar = RoundedCornerShape(
        topStart = 0.dp,
        topEnd = 0.dp,
        bottomStart = 16.dp,
        bottomEnd = 16.dp
    )
    
    // Modales y Overlays
    val BottomSheet = RoundedCornerShape(
        topStart = 28.dp,
        topEnd = 28.dp,
        bottomStart = 0.dp,
        bottomEnd = 0.dp
    )
    val Dialog = RoundedCornerShape(24.dp)
    val Tooltip = RoundedCornerShape(8.dp)
    
    // Elementos de Estado
    val ProgressIndicator = RoundedCornerShape(12.dp)
    val StatusBadge = RoundedCornerShape(16.dp)
    val NotificationCard = RoundedCornerShape(16.dp)
    
    // Streaming y Video
    val VideoContainer = RoundedCornerShape(16.dp)
    val ControlsOverlay = RoundedCornerShape(12.dp)
    val StreamingCard = RoundedCornerShape(20.dp)
}