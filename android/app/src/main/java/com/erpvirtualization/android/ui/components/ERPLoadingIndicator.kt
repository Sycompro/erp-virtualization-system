package com.erpvirtualization.android.ui.components

import androidx.compose.animation.core.*
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.StrokeCap
import androidx.compose.ui.graphics.drawscope.DrawScope
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.erpvirtualization.android.ui.theme.ERPColors
import kotlin.math.cos
import kotlin.math.sin

@Composable
fun ERPLoadingIndicator(
    modifier: Modifier = Modifier,
    size: androidx.compose.ui.unit.Dp = 48.dp,
    strokeWidth: androidx.compose.ui.unit.Dp = 4.dp,
    color: Color = ERPColors.CorporateBlue
) {
    val infiniteTransition = rememberInfiniteTransition(label = "loading_rotation")
    
    val rotation by infiniteTransition.animateFloat(
        initialValue = 0f,
        targetValue = 360f,
        animationSpec = infiniteRepeatable(
            animation = tween(1000, easing = LinearEasing),
            repeatMode = RepeatMode.Restart
        ),
        label = "rotation"
    )
    
    Canvas(
        modifier = modifier.size(size)
    ) {
        drawLoadingCircle(
            rotation = rotation,
            strokeWidth = strokeWidth.toPx(),
            color = color
        )
    }
}

@Composable
fun ERPPulsingDots(
    modifier: Modifier = Modifier,
    dotCount: Int = 3,
    dotSize: androidx.compose.ui.unit.Dp = 8.dp,
    color: Color = ERPColors.CorporateBlue
) {
    val infiniteTransition = rememberInfiniteTransition(label = "pulsing_dots")
    
    Row(
        modifier = modifier,
        horizontalArrangement = Arrangement.spacedBy(8.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        repeat(dotCount) { index ->
            val scale by infiniteTransition.animateFloat(
                initialValue = 0.5f,
                targetValue = 1f,
                animationSpec = infiniteRepeatable(
                    animation = tween(600, delayMillis = index * 200),
                    repeatMode = RepeatMode.Reverse
                ),
                label = "dot_scale_$index"
            )
            
            Canvas(
                modifier = Modifier.size(dotSize)
            ) {
                drawCircle(
                    color = color,
                    radius = (size.minDimension / 2) * scale,
                    center = center
                )
            }
        }
    }
}

@Composable
fun ERPProgressBar(
    progress: Float,
    modifier: Modifier = Modifier,
    height: androidx.compose.ui.unit.Dp = 8.dp,
    backgroundColor: Color = ERPColors.SurfaceSecondary,
    progressColor: Color = ERPColors.CorporateBlue,
    animated: Boolean = true
) {
    val animatedProgress by animateFloatAsState(
        targetValue = if (animated) progress else progress,
        animationSpec = tween(300),
        label = "progress"
    )
    
    Box(
        modifier = modifier
            .height(height)
            .clip(RoundedCornerShape(height / 2))
    ) {
        // Fondo
        Box(
            modifier = Modifier
                .fillMaxSize()
                .clip(RoundedCornerShape(height / 2))
                .background(backgroundColor)
        )
        
        // Progreso
        Box(
            modifier = Modifier
                .fillMaxHeight()
                .fillMaxWidth(animatedProgress.coerceIn(0f, 1f))
                .clip(RoundedCornerShape(height / 2))
                .background(
                    brush = Brush.horizontalGradient(
                        colors = listOf(progressColor, progressColor.copy(alpha = 0.8f))
                    )
                )
        )
    }
}

@Composable
fun ERPLoadingCard(
    title: String,
    subtitle: String? = null,
    progress: Float? = null,
    modifier: Modifier = Modifier
) {
    ERPCard(
        modifier = modifier,
        style = ERPCardStyle.ELEVATED
    ) {
        Column(
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            ERPLoadingIndicator(
                size = 48.dp,
                color = ERPColors.CorporateBlue
            )
            
            Column(
                horizontalAlignment = Alignment.CenterHorizontally,
                verticalArrangement = Arrangement.spacedBy(4.dp)
            ) {
                Text(
                    text = title,
                    style = MaterialTheme.typography.titleMedium,
                    color = ERPColors.TextPrimary,
                    fontWeight = FontWeight.SemiBold
                )
                
                if (subtitle != null) {
                    Text(
                        text = subtitle,
                        style = MaterialTheme.typography.bodyMedium,
                        color = ERPColors.TextSecondary
                    )
                }
            }
            
            if (progress != null) {
                Column(
                    verticalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    ERPProgressBar(
                        progress = progress,
                        modifier = Modifier.fillMaxWidth()
                    )
                    
                    Text(
                        text = "${(progress * 100).toInt()}%",
                        style = MaterialTheme.typography.labelMedium,
                        color = ERPColors.TextSecondary
                    )
                }
            }
        }
    }
}

@Composable
fun ERPSkeletonLoader(
    modifier: Modifier = Modifier,
    height: androidx.compose.ui.unit.Dp = 20.dp,
    cornerRadius: androidx.compose.ui.unit.Dp = 4.dp
) {
    val infiniteTransition = rememberInfiniteTransition(label = "skeleton_shimmer")
    
    val shimmerTranslateAnim by infiniteTransition.animateFloat(
        initialValue = -300f,
        targetValue = 300f,
        animationSpec = infiniteRepeatable(
            animation = tween(1000, easing = LinearEasing),
            repeatMode = RepeatMode.Restart
        ),
        label = "shimmer_translate"
    )
    
    Box(
        modifier = modifier
            .height(height)
            .clip(RoundedCornerShape(cornerRadius))
    ) {
        Canvas(
            modifier = Modifier.fillMaxSize()
        ) {
            val shimmerBrush = Brush.horizontalGradient(
                colors = listOf(
                    ERPColors.SurfaceSecondary,
                    ERPColors.SurfaceSecondary.copy(alpha = 0.5f),
                    ERPColors.SurfaceSecondary
                ),
                startX = shimmerTranslateAnim - 100f,
                endX = shimmerTranslateAnim + 100f
            )
            
            drawRect(
                brush = shimmerBrush,
                size = size
            )
        }
    }
}

private fun DrawScope.drawLoadingCircle(
    rotation: Float,
    strokeWidth: Float,
    color: Color
) {
    val center = Offset(size.width / 2, size.height / 2)
    val radius = (size.minDimension - strokeWidth) / 2
    
    // Dibujar círculo de fondo
    drawCircle(
        color = color.copy(alpha = 0.2f),
        radius = radius,
        center = center,
        style = Stroke(width = strokeWidth, cap = StrokeCap.Round)
    )
    
    // Dibujar arco de progreso
    val sweepAngle = 270f
    val startAngle = rotation - 90f
    
    drawArc(
        color = color,
        startAngle = startAngle,
        sweepAngle = sweepAngle,
        useCenter = false,
        style = Stroke(width = strokeWidth, cap = StrokeCap.Round),
        topLeft = Offset(
            center.x - radius,
            center.y - radius
        ),
        size = androidx.compose.ui.geometry.Size(radius * 2, radius * 2)
    )
}