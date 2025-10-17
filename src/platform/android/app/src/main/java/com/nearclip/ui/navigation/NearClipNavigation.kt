package com.nearclip.ui.navigation

import androidx.compose.foundation.layout.padding
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import com.nearclip.ui.screens.HomeScreen
import com.nearclip.ui.screens.DeviceListScreen
import com.nearclip.ui.screens.SettingsScreen
import com.nearclip.ui.theme.NearClipTheme

/**
 * NearClip导航组件
 */
@Composable
fun NearClipNavigation(
    navController: NavHostController = rememberNavController()
) {
    NearClipTheme {
        Scaffold(
            bottomBar = {
                BottomNavigationBar(navController = navController)
            }
        ) { paddingValues ->
            NavHost(
                navController = navController,
                startDestination = Screen.Home.route,
                modifier = Modifier.padding(paddingValues)
            ) {
                composable(Screen.Home.route) {
                    HomeScreen(
                        onNavigateToDevices = { navController.navigate(Screen.DeviceList.route) }
                    )
                }
                composable(Screen.DeviceList.route) {
                    DeviceListScreen(
                        onNavigateBack = { navController.popBackStack() }
                    )
                }
                composable(Screen.Settings.route) {
                    SettingsScreen(
                        onNavigateBack = { navController.popBackStack() }
                    )
                }
            }
        }
    }
}

/**
 * 导航屏幕枚举
 */
sealed class Screen(val route: String) {
    object Home : Screen("home")
    object DeviceList : Screen("devices")
    object Settings : Screen("settings")
}

/**
 * 底部导航栏
 */
@Composable
private fun BottomNavigationBar(navController: NavHostController) {
    val items = listOf(
        Screen.Home,
        Screen.DeviceList,
        Screen.Settings
    )

    val navBackStackEntry by navController.currentBackStackEntryAsState()
    val currentRoute = navBackStackEntry?.destination?.route

    BottomAppBar(
        actions = {
            items.forEach { screen ->
                NavigationBarItem(
                    icon = { Icon(screen.icon, contentDescription = null) },
                    label = { Text(screen.title) },
                    selected = currentRoute == screen.route,
                    onClick = {
                        if (currentRoute != screen.route) {
                            navController.navigate(screen.route) {
                                popUpTo(navController.graph.startDestinationId)
                                saveState = true
                            }
                        }
                    }
                )
            }
        }
    )
}

/**
 * 屏幕图标和标题
 */
private val Screen.icon: androidx.compose.ui.graphics.vector.ImageVector
    @Composable
    get() = when (this) {
        Screen.Home -> androidx.compose.material.icons.Icons.Home
        Screen.DeviceList -> androidx.compose.material.icons.Icons.Devices
        Screen.Settings -> androidx.compose.material.icons.Icons.Settings
    }

private val Screen.title: String
    get() = when (this) {
        Screen.Home -> "首页"
        Screen.DeviceList -> "设备"
        Screen.Settings -> "设置"
    }
}