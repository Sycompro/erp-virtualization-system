package com.erpvirtualization.android.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.erpvirtualization.android.ui.theme.ERPColors

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ERPTopBar(
    title: String,
    modifier: Modifier = Modifier,
    subtitle: String? = null,
    navigationIcon: ImageVector? = null,
    onNavigationClick: (() -> Unit)? = null,
    actions: @Composable RowScope.() -> Unit = {},
    style: ERPTopBarStyle = ERPTopBarStyle.GRADIENT
) {
    TopAppBar(
        title = {
            Column {
                Text(
                    text = title,
                    style = MaterialTheme.typography.titleLarge,
                    fontWeight = FontWeight.Bold,
                    color = when (style) {
                        ERPTopBarStyle.GRADIENT, ERPTopBarStyle.SOLID -> ERPColors.TextOnPrimary
                        ERPTopBarStyle.TRANSPARENT -> ERPColors.TextPrimary
                    }
                )
                
                if (subtitle != null) {
                    Text(
                        text = subtitle,
                        style = MaterialTheme.typography.bodySmall,
                        color = when (style) {
                            ERPTopBarStyle.GRADIENT, ERPTopBarStyle.SOLID -> ERPColors.TextOnPrimary.copy(alpha = 0.8f)
                            ERPTopBarStyle.TRANSPARENT -> ERPColors.TextSecondary
                        }
                    )
                }
            }
        },
        modifier = modifier.then(
            if (style == ERPTopBarStyle.GRADIENT) {
                Modifier.background(
                    brush = Brush.horizontalGradient(ERPColors.GradientPrimary)
                )
            } else Modifier
        ),
        navigationIcon = {
            if (navigationIcon != null && onNavigationClick != null) {
                IconButton(onClick = onNavigationClick) {
                    Icon(
                        imageVector = navigationIcon,
                        contentDescription = "Navegación",
                        tint = when (style) {
                            ERPTopBarStyle.GRADIENT, ERPTopBarStyle.SOLID -> ERPColors.TextOnPrimary
                            ERPTopBarStyle.TRANSPARENT -> ERPColors.TextPrimary
                        }
                    )
                }
            }
        },
        actions = actions,
        colors = TopAppBarDefaults.topAppBarColors(
            containerColor = when (style) {
                ERPTopBarStyle.GRADIENT -> Color.Transparent
                ERPTopBarStyle.SOLID -> ERPColors.CorporateBlue
                ERPTopBarStyle.TRANSPARENT -> Color.Transparent
            }
        )
    )
}

@Composable
fun ERPConnectionStatusBar(
    connectionState: String,
    serverInfo: String? = null,
    modifier: Modifier = Modifier,
    onStatusClick: (() -> Unit)? = null
) {
    val (statusColor, statusIcon) = when (connectionState.lowercase()) {
        "conectado", "connected" -> Pair(ERPColors.SuccessGreen, Icons.Default.CheckCircle)
        "conectando", "connecting" -> Pair(ERPColors.WarningAmber, Icons.Default.Sync)
        "desconectado", "disconnected" -> Pair(ERPColors.ExecutiveGrayLight, Icons.Default.Circle)
        "error" -> Pair(ERPColors.ErrorRed, Icons.Default.Error)
        else -> Pair(ERPColors.ExecutiveGrayLight, Icons.Default.Circle)
    }
    
    Surface(
        modifier = modifier
            .fillMaxWidth()
            .clip(RoundedCornerShape(bottomStart = 16.dp, bottomEnd = 16.dp)),
        color = statusColor.copy(alpha = 0.1f),
        onClick = onStatusClick ?: {}
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 16.dp, vertical = 8.dp),
            horizontalArrangement = Arrangement.spacedBy(8.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Icon(
                imageVector = statusIcon,
                contentDescription = null,
                modifier = Modifier.size(16.dp),
                tint = statusColor
            )
            
            Text(
                text = connectionState,
                style = MaterialTheme.typography.labelMedium,
                color = statusColor,
                fontWeight = FontWeight.Medium
            )
            
            if (serverInfo != null) {
                Text(
                    text = "•",
                    style = MaterialTheme.typography.labelMedium,
                    color = ERPColors.TextTertiary
                )
                
                Text(
                    text = serverInfo,
                    style = MaterialTheme.typography.labelSmall,
                    color = ERPColors.TextSecondary
                )
            }
            
            Spacer(modifier = Modifier.weight(1f))
            
            if (onStatusClick != null) {
                Icon(
                    imageVector = Icons.Default.KeyboardArrowRight,
                    contentDescription = null,
                    modifier = Modifier.size(16.dp),
                    tint = ERPColors.TextTertiary
                )
            }
        }
    }
}

@Composable
fun ERPActionButton(
    icon: ImageVector,
    contentDescription: String,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
    badge: String? = null,
    tint: Color = ERPColors.TextOnPrimary
) {
    Box(modifier = modifier) {
        IconButton(
            onClick = onClick,
            enabled = enabled
        ) {
            Icon(
                imageVector = icon,
                contentDescription = contentDescription,
                tint = if (enabled) tint else tint.copy(alpha = 0.5f)
            )
        }
        
        if (badge != null) {
            Badge(
                modifier = Modifier.align(Alignment.TopEnd),
                containerColor = ERPColors.ErrorRed
            ) {
                Text(
                    text = badge,
                    style = MaterialTheme.typography.labelSmall,
                    color = ERPColors.TextOnPrimary
                )
            }
        }
    }
}

enum class ERPTopBarStyle {
    GRADIENT,
    SOLID,
    TRANSPARENT
}