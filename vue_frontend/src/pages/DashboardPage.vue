<template>
    <v-app>
      <HeaderComponent />
      <v-main>
        <v-container fluid>
          <v-row>
            <v-col cols="2">
              <SidebarComponent />
            </v-col>
            <v-col cols="10">
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
  .v-container {
    display: flex;
    min-height: 100vh;
  }
</style>
  ./DashboardPage.vue