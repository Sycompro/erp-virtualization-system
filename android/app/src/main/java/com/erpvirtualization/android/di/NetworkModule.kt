package com.erpvirtualization.android.di

import android.content.Context
import com.erpvirtualization.android.BuildConfig
import com.erpvirtualization.android.data.api.ERPApiService
import com.erpvirtualization.android.data.api.ERPLocalService
import com.erpvirtualization.android.data.local.PreferencesManager
import com.erpvirtualization.android.webrtc.WebRTCManager
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.components.SingletonComponent
import kotlinx.serialization.json.Json
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import retrofit2.Retrofit
import com.jakewharton.retrofit2.converter.kotlinx.serialization.asConverterFactory
import timber.log.Timber
import java.util.concurrent.TimeUnit
import javax.inject.Singleton

@Module
@InstallIn(SingletonComponent::class)
object NetworkModule {
    
    @Provides
    @Singleton
    fun provideJson(): Json = Json {
        ignoreUnknownKeys = true
        coerceInputValues = true
    }
    
    @Provides
    @Singleton
    fun provideOkHttpClient(
        preferencesManager: PreferencesManager
    ): OkHttpClient {
        val loggingInterceptor = HttpLoggingInterceptor { message ->
            Timber.tag("HTTP").d(message)
        }.apply {
            level = if (BuildConfig.DEBUG) {
                HttpLoggingInterceptor.Level.BODY
            } else {
                HttpLoggingInterceptor.Level.NONE
            }
        }
        
        return OkHttpClient.Builder()
            .addInterceptor(loggingInterceptor)
            .addInterceptor { chain ->
                val originalRequest = chain.request()
                val token = preferencesManager.getAuthToken()
                
                val newRequest = if (token != null) {
                    originalRequest.newBuilder()
                        .header("Authorization", "Bearer $token")
                        .header("Content-Type", "application/json")
                        .header("Accept", "application/json")
                        .build()
                } else {
                    originalRequest.newBuilder()
                        .header("Content-Type", "application/json")
                        .header("Accept", "application/json")
                        .build()
                }
                
                chain.proceed(newRequest)
            }
            .connectTimeout(30, TimeUnit.SECONDS)
            .readTimeout(30, TimeUnit.SECONDS)
            .writeTimeout(30, TimeUnit.SECONDS)
            .build()
    }
    
    @Provides
    @Singleton
    fun provideRetrofit(
        okHttpClient: OkHttpClient,
        json: Json
    ): Retrofit {
        // Usar la URL configurada en BuildConfig
        val baseUrl = BuildConfig.RAILWAY_API_URL
        
        return Retrofit.Builder()
            .baseUrl(baseUrl)
            .client(okHttpClient)
            .addConverterFactory(
                json.asConverterFactory("application/json".toMediaType())
            )
            .build()
    }
    
    @Provides
    @Singleton
    fun provideERPApiService(retrofit: Retrofit): ERPApiService {
        return retrofit.create(ERPApiService::class.java)
    }
    
    @Provides
    @Singleton
    fun provideERPLocalService(retrofit: Retrofit): ERPLocalService {
        return retrofit.create(ERPLocalService::class.java)
    }
    
    @Provides
    @Singleton
    fun provideWebRTCManager(
        @ApplicationContext context: Context
    ): WebRTCManager {
        return WebRTCManager(context)
    }
    
    @Provides
    @Singleton
    fun providePreferencesManager(
        @ApplicationContext context: Context
    ): PreferencesManager {
        return PreferencesManager(context)
    }
}