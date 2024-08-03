<template>
    <v-app-bar app color="primary" dark>
      <v-toolbar-title>SecureMQ</v-toolbar-title>
      <v-spacer></v-spacer>
      <v-btn text @click="$router.push('/')">Logout</v-btn>
    </v-app-bar>
</template>
  
<script>
  export default {
    name: 'HeaderComponent'
  };
</script>
  
<style scoped>
  .v-app-bar {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
  }
</style>
  