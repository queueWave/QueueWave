<template>
    <v-container>
    <h1>All Packages</h1>
    </v-container>
    <v-container>
    <PackageTable />
    </v-container>
    
</template>
  
<script>
  import PackageTable from '../components/PackageTable.vue';
  
  export default {
    name: 'AllPackages',
    components: {
      PackageTable
    }
  };
</script>
  
<style scoped>
</style>
  