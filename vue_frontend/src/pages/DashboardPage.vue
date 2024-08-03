<template>
    <v-app>
      <HeaderComponent />
      <v-main>
        <v-container fluid class="dashboard-container">
          <v-row no-gutters>
            <v-col cols="2" class="sidebar">
              <SidebarComponent />
            </v-col>
            <v-col  class="content">
              <router-view></router-view>
            </v-col>
          </v-row>
        </v-container>
      </v-main>
    </v-app>
</template>
  
<script>
  import HeaderComponent from '../components/HeaderComponent.vue';
  import SidebarComponent from '../components/SidebarComponent.vue';
  
  export default {
    name: 'DashboardPage',
    components: {
      HeaderComponent,
      SidebarComponent
    }
  };
</script>
  
<style scoped>
  .dashboard-container {
    padding: 0;
    height: calc(100vh - 64px); 
    width: 100%; 
    flex-direction: row;
    width:100%;
  }
  
  .sidebar {
    background-color: #333;
    height: 100%;
    min-height: calc(100vh - 64px); 
    flex: 0 ; 
  }
  
  .content {
    padding: 20px;
    height: 100%;
    width: 100%;
  }
</style>
  