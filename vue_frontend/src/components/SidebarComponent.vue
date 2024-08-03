<template>
    <v-navigation-drawer app>
      <v-list dense>
        <v-list-item @click="$router.push('/dashboard')" class="nav-item">
          <v-list-item-icon>
            <v-icon>mdi-view-dashboard</v-icon>
          </v-list-item-icon>
          <v-list-item-content>
            <v-list-item-title>Dashboard</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
        <v-list-item @click="$router.push('/dashboard/all-packages')" class="nav-item">
          <v-list-item-icon>
            <v-icon>mdi-package-variant-closed</v-icon>
          </v-list-item-icon>
          <v-list-item-content>
            <v-list-item-title>All Packages</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>
</template>
  
<script>
  export default {
    name: 'SidebarComponent'
  };
</script>
  
<style scoped>
  .nav-item {
    cursor: pointer;
    transition: background-color 0.3s ease;
  }
  
  .nav-item:hover {
    background-color: #f5f5f5;
  }
</style>
  