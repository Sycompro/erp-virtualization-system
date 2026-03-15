package com.erpvirtualization.android.ui.theme

import android.app.Activity
import android.os.Build
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.dynamicDarkColorScheme
import androidx.compose.material3.dynamicLightColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.SideEffect
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalView
import androidx.core.view.WindowCompat

private val DarkColorScheme = darkColorScheme(
    primary = ERPColors.CorporateBlueLight,
    onPrimary = ERPColors.TextOnPrimary,
    primaryContainer = ERPColors.CorporateBlueDark,
    onPrimaryContainer = ERPColors.SoftLavender,
    
    secondary = ERPColors.EnterpriseGreen,
    onSecondary = ERPColors.TextOnPrimary,
    secondaryContainer = ERPColors.EnterpriseGreenDark,
    onSecondaryContainer = ERPColors.SoftMint,
    
    tertiary = ERPColors.AccentPurple,
    onTertiary = ERPColors.TextOnPrimary,
    tertiaryContainer = ERPColors.AccentTeal,
    onTertiaryContainer = ERPColors.SoftSky,
    
    error = ERPColors.ErrorRed,
    errorContainer = ERPColors.SoftRose,
    onError = ERPColors.TextOnPrimary,
    onErrorContainer = ERPColors.TextPrimary,
    
    background = ERPColors.ExecutiveGrayDark,
    onBackground = ERPColors.TextOnPrimary,
    
    surface = ERPColors.ExecutiveGray,
    onSurface = ERPColors.TextOnPrimary,
    surfaceVariant = ERPColors.ExecutiveGrayLight,
    onSurfaceVariant = ERPColors.TextSecondary,
    
    outline = ERPColors.TextTertiary,
    inverseOnSurface = ERPColors.TextPrimary,
    inverseSurface = ERPColors.SurfacePrimary,
    inversePrimary = ERPColors.CorporateBlue,
)

private val LightColorScheme = lightColorScheme(
    primary = ERPColors.CorporateBlue,
    onPrimary = ERPColors.TextOnPrimary,
    primaryContainer = ERPColors.SoftLavender,
    onPrimaryContainer = ERPColors.CorporateBlueDark,
    
    secondary = ERPColors.EnterpriseGreen,
    onSecondary = ERPColors.TextOnPrimary,
    secondaryContainer = ERPColors.SoftMint,
    onSecondaryContainer = ERPColors.EnterpriseGreenDark,
    
    tertiary = ERPColors.AccentPurple,
    onTertiary = ERPColors.TextOnPrimary,
    tertiaryContainer = ERPColors.SoftSky,
    onTertiaryContainer = ERPColors.AccentTeal,
    
    error = ERPColors.ErrorRed,
    errorContainer = ERPColors.SoftRose,
    onError = ERPColors.TextOnPrimary,
    onErrorContainer = ERPColors.TextPrimary,
    
    background = ERPColors.SurfacePrimary,
    onBackground = ERPColors.TextPrimary,
    
    surface = ERPColors.SurfaceCard,
    onSurface = ERPColors.TextPrimary,
    surfaceVariant = ERPColors.SurfaceSecondary,
    onSurfaceVariant = ERPColors.TextSecondary,
    
    outline = ERPColors.TextTertiary,
    inverseOnSurface = ERPColors.TextOnPrimary,
    inverseSurface = ERPColors.ExecutiveGray,
    inversePrimary = ERPColors.CorporateBlueLight,
)

@Composable
fun ERPVirtualizationTheme(
    darkTheme: Boolean = isSystemInDarkTheme(),
    // Dynamic color is available on Android 12+
    dynamicColor: Boolean = false,
    content: @Composable () -> Unit
) {
    val colorScheme = when {
        dynamicColor && Build.VERSION.SDK_INT >= Build.VERSION_CODES.S -> {
            val context = LocalContext.current
            if (darkTheme) dynamicDarkColorScheme(context) else dynamicLightColorScheme(context)
        }

        darkTheme -> DarkColorScheme
        else -> LightColorScheme
    }
    
    val view = LocalView.current
    if (!view.isInEditMode) {
        SideEffect {
            val window = (view.context as Activity).window
            window.statusBarColor = colorScheme.primary.toArgb()
            WindowCompat.getInsetsController(window, view).isAppearanceLightStatusBars = darkTheme
        }
    }

    MaterialTheme(
        colorScheme = colorScheme,
        typography = Typography,
        shapes = ERPShapes,
        content = content
    )
}