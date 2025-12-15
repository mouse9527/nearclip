package com.nearclip.ui.navigation

import androidx.compose.runtime.Composable
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import com.nearclip.ui.screens.HomeScreen
import com.nearclip.ui.screens.PairingScreen
import com.nearclip.ui.screens.SettingsScreen

@Composable
fun NearClipNavHost(
    navController: NavHostController,
    startDestination: String = NavRoutes.Home.route
) {
    NavHost(
        navController = navController,
        startDestination = startDestination
    ) {
        composable(NavRoutes.Home.route) {
            HomeScreen(
                onNavigateToPairing = {
                    navController.navigate(NavRoutes.Pairing.route)
                },
                onNavigateToSettings = {
                    navController.navigate(NavRoutes.Settings.route)
                }
            )
        }

        composable(NavRoutes.Pairing.route) {
            PairingScreen(
                onNavigateBack = {
                    navController.popBackStack()
                }
            )
        }

        composable(NavRoutes.Settings.route) {
            SettingsScreen(
                onNavigateBack = {
                    navController.popBackStack()
                }
            )
        }
    }
}
