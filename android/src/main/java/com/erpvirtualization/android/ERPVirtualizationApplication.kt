package com.erpvirtualization.android

import android.app.Application
import dagger.hilt.android.HiltAndroidApp
import timber.log.Timber

@HiltAndroidApp
class ERPVirtualizationApplication : Application() {
    
    override fun onCreate() {
        super.onCreate()
        
        // Inicializar Timber para logging
        if (BuildConfig.DEBUG) {
            Timber.plant(Timber.DebugTree())
        } else {
            // En producción, usar un logger que no imprima logs sensibles
            Timber.plant(object : Timber.Tree() {
                override fun log(priority: Int, tag: String?, message: String, t: Throwable?) {
                    // Solo logs de error en producción
                    if (priority >= android.util.Log.ERROR) {
                        // Aquí podrías enviar a un servicio de crash reporting
                        // como Firebase Crashlytics
                    }
                }
            })
        }
        
        Timber.d("🚀 ERP Virtualization App iniciada")
        Timber.d("📱 Versión: ${BuildConfig.VERSION_NAME} (${BuildConfig.VERSION_CODE})")
        Timber.d("🔧 Debug: ${BuildConfig.DEBUG}")
    }
}