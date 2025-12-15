package com.nearclip.ui.navigation

sealed class NavRoutes(val route: String) {
    object Home : NavRoutes("home")
    object Pairing : NavRoutes("pairing")
    object Settings : NavRoutes("settings")
}
