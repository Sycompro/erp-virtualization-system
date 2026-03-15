package com.erpvirtualization.android.ui.components

import androidx.compose.animation.animateColorAsState
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.animation.core.tween
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.ripple.rememberRipple
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.scale
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.erpvirtualization.android.ui.theme.ERPColors
import com.erpvirtualization.android.ui.theme.ERPCustomShapes

enum class ERPCardStyle {
    ELEVATED,
    FLAT,
    OUTLINED,
    GRADIENT,
    SUCCESS,
    WARNING,
    ERROR,
    INFO
}

@Composable
fun ERPCard(
    modifier: Modifier = Modifier,
    style: ERPCardStyle = ERPCardStyle.ELEVATED,
    onClick: (() -> Unit)? = null,
    enabled: Boolean = true,
    content: @Composable ColumnScope.() -> Unit
) {
    var isPressed by remember { mutableStateOf(false) }
    
    val scale by animateFloatAsState(
        targetValue = if (isPressed && onClick != null) 0.98f else 1f,
        animationSpec = tween(100),
        label = "card_scale"
    )
    
    val (backgroundColor, borderColor, shadowElevation) = getCardColors(style)
    
    val animatedBackgroundColor by animateColorAsState(
        targetValue = backgroundColor,
        animationSpec = tween(200),
        label = "card_background"
    )
    
    val interactionSource = remember { MutableInteractionSource() }
    
    Card(
        modifier = modifier
            .scale(scale)
            .then(
                if (onClick != null) {
                    Modifier.clickable(
                        interactionSource = interactionSource,
                        indication = rememberRipple(
                            color = ERPColors.CorporateBlue.copy(alpha = 0.1f)
                        ),
                        enabled = enabled,
                        onClick = {
                            isPressed = true
                            onClick()
                            isPressed = false
                        }
                    )
                } else Modifier
            ),
        shape = ERPCustomShapes.CardElevated,
        colors = CardDefaults.cardColors(
            containerColor = if (style == ERPCardStyle.GRADIENT) {
                Color.Transparent
            } else {
                animatedBackgroundColor
            }
        ),
        elevation = CardDefaults.cardElevation(
            defaultElevation = shadowElevation
        ),
        border = if (borderColor != Color.Transparent) {
            androidx.compose.foundation.BorderStroke(1.dp, borderColor)
        } else null
    ) {
        Box(
            modifier = if (style == ERPCardStyle.GRADIENT) {
                Modifier
                    .fillMaxWidth()
                    .background(
                        brush = Brush.linearGradient(ERPColors.GradientSoft),
                        shape = ERPCustomShapes.CardElevated
                    )
            } else Modifier.fillMaxWidth()
        ) {
            Column(
                modifier = Modifier.padding(20.dp),
                content = content
            )
        }
    }
}

@Composable
fun ERPInfoCard(
    title: String,
    subtitle: String? = null,
    value: String,
    icon: ImageVector,
    modifier: Modifier = Modifier,
    style: ERPCardStyle = ERPCardStyle.ELEVATED,
    onClick: (() -> Unit)? = null
) {
    ERPCard(
        modifier = modifier,
        style = style,
        onClick = onClick
    ) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Column(
                modifier = Modifier.weight(1f)
            ) {
                Text(
                    text = title,
                    style = MaterialTheme.typography.titleMedium,
                    color = ERPColors.TextPrimary,
                    fontWeight = FontWeight.SemiBold
                )
                
                if (subtitle != null) {
                    Spacer(modifier = Modifier.height(4.dp))
                    Text(
                        text = subtitle,
                        style = MaterialTheme.typography.bodySmall,
                        color = ERPColors.TextSecondary
                    )
                }
                
                Spacer(modifier = Modifier.height(8.dp))
                
                Text(
                    text = value,
                    style = MaterialTheme.typography.headlineSmall,
                    color = ERPColors.CorporateBlue,
                    fontWeight = FontWeight.Bold
                )
            }
            
            Box(
                modifier = Modifier
                    .size(56.dp)
                    .background(
                        color = ERPColors.SoftLavender,
                        shape = RoundedCornerShape(16.dp)
                    ),
                contentAlignment = Alignment.Center
            ) {
                Icon(
                    imageVector = icon,
                    contentDescription = null,
                    modifier = Modifier.size(28.dp),
                    tint = ERPColors.CorporateBlue
                )
            }
        }
    }
}

@Composable
fun ERPStatusCard(
    title: String,
    status: String,
    statusColor: Color,
    description: String? = null,
    modifier: Modifier = Modifier,
    onClick: (() -> Unit)? = null
) {
    ERPCard(
        modifier = modifier,
        style = ERPCardStyle.ELEVATED,
        onClick = onClick
    ) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.Top
        ) {
            Column(
                modifier = Modifier.weight(1f)
            ) {
                Text(
                    text = title,
                    style = MaterialTheme.typography.titleMedium,
                    color = ERPColors.TextPrimary,
                    fontWeight = FontWeight.SemiBold
                )
                
                if (description != null) {
                    Spacer(modifier = Modifier.height(4.dp))
                    Text(
                        text = description,
                        style = MaterialTheme.typography.bodySmall,
                        color = ERPColors.TextSecondary
                    )
                }
            }
            
            Box(
                modifier = Modifier
                    .background(
                        color = statusColor.copy(alpha = 0.1f),
                        shape = RoundedCornerShape(12.dp)
                    )
                    .padding(horizontal = 12.dp, vertical = 6.dp)
            ) {
                Text(
                    text = status,
                    style = MaterialTheme.typography.labelMedium,
                    color = statusColor,
                    fontWeight = FontWeight.Medium
                )
            }
        }
    }
}

@Composable
private fun getCardColors(style: ERPCardStyle): Triple<Color, Color, androidx.compose.ui.unit.Dp> {
    return when (style) {
        ERPCardStyle.ELEVATED -> Triple(
            ERPColors.SurfaceCard,
            Color.Transparent,
            8.dp
        )
        ERPCardStyle.FLAT -> Triple(
            ERPColors.SurfaceSecondary,
            Color.Transparent,
            0.dp
        )
        ERPCardStyle.OUTLINED -> Triple(
            ERPColors.SurfaceCard,
            ERPColors.ExecutiveGrayLight,
            2.dp
        )
        ERPCardStyle.GRADIENT -> Triple(
            Color.Transparent, // Será sobrescrito por el gradiente
            Color.Transparent,
            4.dp
        )
        ERPCardStyle.SUCCESS -> Triple(
            ERPColors.SoftMint,
            ERPColors.EnterpriseGreen.copy(alpha = 0.3f),
            4.dp
        )
        ERPCardStyle.WARNING -> Triple(
            ERPColors.SoftPeach,
            ERPColors.WarningAmber.copy(alpha = 0.3f),
            4.dp
        )
        ERPCardStyle.ERROR -> Triple(
            ERPColors.SoftRose,
            ERPColors.ErrorRed.copy(alpha = 0.3f),
            4.dp
        )
        ERPCardStyle.INFO -> Triple(
            ERPColors.SoftSky,
            ERPColors.InfoBlue.copy(alpha = 0.3f),
            4.dp
        )
    }
}