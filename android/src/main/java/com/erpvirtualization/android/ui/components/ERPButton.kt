package com.erpvirtualization.android.ui.components

import androidx.compose.animation.animateColorAsState
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.animation.core.tween
import androidx.compose.foundation.background
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
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.erpvirtualization.android.ui.theme.ERPColors
import com.erpvirtualization.android.ui.theme.ERPCustomShapes

enum class ERPButtonStyle {
    PRIMARY,
    SECONDARY,
    OUTLINE,
    TEXT,
    SUCCESS,
    WARNING,
    ERROR,
    GRADIENT
}

enum class ERPButtonSize {
    SMALL,
    MEDIUM,
    LARGE,
    EXTRA_LARGE
}

@Composable
fun ERPButton(
    text: String,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    style: ERPButtonStyle = ERPButtonStyle.PRIMARY,
    size: ERPButtonSize = ERPButtonSize.MEDIUM,
    enabled: Boolean = true,
    loading: Boolean = false,
    icon: ImageVector? = null,
    iconPosition: IconPosition = IconPosition.START
) {
    var isPressed by remember { mutableStateOf(false) }
    
    val scale by animateFloatAsState(
        targetValue = if (isPressed) 0.95f else 1f,
        animationSpec = tween(100),
        label = "button_scale"
    )
    
    val (backgroundColor, contentColor, borderColor) = getButtonColors(style, enabled)
    
    val animatedBackgroundColor by animateColorAsState(
        targetValue = backgroundColor,
        animationSpec = tween(200),
        label = "background_color"
    )
    
    val (height, horizontalPadding, fontSize, iconSize) = getButtonDimensions(size)
    
    val interactionSource = remember { MutableInteractionSource() }
    
    Box(
        modifier = modifier
            .height(height)
            .scale(scale)
            .clip(ERPCustomShapes.ButtonPrimary)
            .background(
                if (style == ERPButtonStyle.GRADIENT) {
                    Brush.horizontalGradient(ERPColors.GradientPrimary)
                } else {
                    Brush.horizontalGradient(listOf(animatedBackgroundColor, animatedBackgroundColor))
                }
            )
            .clickable(
                interactionSource = interactionSource,
                indication = rememberRipple(
                    color = contentColor.copy(alpha = 0.2f)
                ),
                enabled = enabled && !loading,
                onClick = {
                    isPressed = true
                    onClick()
                    isPressed = false
                }
            )
            .padding(horizontal = horizontalPadding, vertical = 0.dp),
        contentAlignment = Alignment.Center
    ) {
        if (loading) {
            CircularProgressIndicator(
                modifier = Modifier.size(iconSize),
                color = contentColor,
                strokeWidth = 2.dp
            )
        } else {
            Row(
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.Center
            ) {
                if (icon != null && iconPosition == IconPosition.START) {
                    Icon(
                        imageVector = icon,
                        contentDescription = null,
                        modifier = Modifier.size(iconSize),
                        tint = contentColor
                    )
                    Spacer(modifier = Modifier.width(8.dp))
                }
                
                Text(
                    text = text,
                    color = contentColor,
                    fontSize = fontSize,
                    fontWeight = FontWeight.SemiBold,
                    letterSpacing = 0.5.sp
                )
                
                if (icon != null && iconPosition == IconPosition.END) {
                    Spacer(modifier = Modifier.width(8.dp))
                    Icon(
                        imageVector = icon,
                        contentDescription = null,
                        modifier = Modifier.size(iconSize),
                        tint = contentColor
                    )
                }
            }
        }
    }
}

@Composable
private fun getButtonColors(
    style: ERPButtonStyle,
    enabled: Boolean
): Triple<Color, Color, Color> {
    return when (style) {
        ERPButtonStyle.PRIMARY -> Triple(
            if (enabled) ERPColors.CorporateBlue else ERPColors.ExecutiveGrayLight,
            ERPColors.TextOnPrimary,
            Color.Transparent
        )
        ERPButtonStyle.SECONDARY -> Triple(
            if (enabled) ERPColors.EnterpriseGreen else ERPColors.ExecutiveGrayLight,
            ERPColors.TextOnPrimary,
            Color.Transparent
        )
        ERPButtonStyle.OUTLINE -> Triple(
            Color.Transparent,
            if (enabled) ERPColors.CorporateBlue else ERPColors.ExecutiveGrayLight,
            if (enabled) ERPColors.CorporateBlue else ERPColors.ExecutiveGrayLight
        )
        ERPButtonStyle.TEXT -> Triple(
            Color.Transparent,
            if (enabled) ERPColors.CorporateBlue else ERPColors.ExecutiveGrayLight,
            Color.Transparent
        )
        ERPButtonStyle.SUCCESS -> Triple(
            if (enabled) ERPColors.SuccessGreen else ERPColors.ExecutiveGrayLight,
            ERPColors.TextOnPrimary,
            Color.Transparent
        )
        ERPButtonStyle.WARNING -> Triple(
            if (enabled) ERPColors.WarningAmber else ERPColors.ExecutiveGrayLight,
            ERPColors.TextOnPrimary,
            Color.Transparent
        )
        ERPButtonStyle.ERROR -> Triple(
            if (enabled) ERPColors.ErrorRed else ERPColors.ExecutiveGrayLight,
            ERPColors.TextOnPrimary,
            Color.Transparent
        )
        ERPButtonStyle.GRADIENT -> Triple(
            ERPColors.CorporateBlue, // Será sobrescrito por el gradiente
            ERPColors.TextOnPrimary,
            Color.Transparent
        )
    }
}

@Composable
private fun getButtonDimensions(size: ERPButtonSize): ButtonDimensions {
    return when (size) {
        ERPButtonSize.SMALL -> ButtonDimensions(
            height = 36.dp,
            horizontalPadding = 16.dp,
            fontSize = 12.sp,
            iconSize = 16.dp
        )
        ERPButtonSize.MEDIUM -> ButtonDimensions(
            height = 48.dp,
            horizontalPadding = 24.dp,
            fontSize = 14.sp,
            iconSize = 20.dp
        )
        ERPButtonSize.LARGE -> ButtonDimensions(
            height = 56.dp,
            horizontalPadding = 32.dp,
            fontSize = 16.sp,
            iconSize = 24.dp
        )
        ERPButtonSize.EXTRA_LARGE -> ButtonDimensions(
            height = 64.dp,
            horizontalPadding = 40.dp,
            fontSize = 18.sp,
            iconSize = 28.dp
        )
    }
}

private data class ButtonDimensions(
    val height: androidx.compose.ui.unit.Dp,
    val horizontalPadding: androidx.compose.ui.unit.Dp,
    val fontSize: androidx.compose.ui.unit.TextUnit,
    val iconSize: androidx.compose.ui.unit.Dp
)

enum class IconPosition {
    START,
    END
}