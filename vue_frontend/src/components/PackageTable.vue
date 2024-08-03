<template>
    <v-container fluid>
      <v-data-table
        :headers="headers"
        :items="packages"
        item-value="id"
        @click:row="showDetails"
        class="elevation-1"
        dense
      >
      <template v-slot:item="{ item }">
        <tr @click="showDetails(item)">
          <td>{{ item.id }}</td>
          <td>{{ new Date(item.received).toLocaleString() }}</td>
          <td>{{ item.size }} bytes</td>
          <td>{{ item.message.message.sender.sender }}</td>
          <td>{{ item.message.queue }}</td>
          <td>{{ new Date(item.message.message.sender.timestamp).toLocaleString() }}</td>
        </tr>
      </template>
      </v-data-table>
     
  
      <v-dialog v-model="dialog" max-width="600">
        <PackageInfoComponent v-if="selectedPackageId" :id="selectedPackageId" @close="dialog = false" />
      </v-dialog>
    </v-container>
</template>
  
<script>
  import axios from 'axios';
  import PackageInfoComponent from './PackageInfoComponent.vue';
  
  export default {
    name: 'PackageTable',
    components: {
      PackageInfoComponent
    },
    data() {
      return {
        headers: [
          { title: 'ID', value: 'id' },
          { title: 'Received', value: 'received' },
          { title: 'Size', value: 'size' },
          { title: 'Sender', value: 'message.message.sender.sender' },
          { title: 'Queue', value: 'message.queue' },
          { title: 'Created', value: 'message.message.sender.timestamp' },
        ],
        packages: [],
        dialog: false,
        selectedPackageId: null,
      };
    },
    mounted() {
      this.fetchPackages();
      setInterval(this.fetchPackages, 500); // Reload every 0.5 second
    },
    methods: {
      async fetchPackages() {
        try {
          const response = await axios.get('http://localhost:8080/api/all_packages');
          this.packages = response.data;
        } catch (error) {
          console.error('Error fetching packages:', error);
        }
      },
      showDetails(item) {
        console.log(item); 
        this.selectedPackageId = item.id; 
        this.dialog = true;
      }
    }
  };
</script>
  
<style scoped>
  .v-container {
    padding: 0;
    margin: 0;
    width: 100%;
  }
  
  .v-data-table {
    border-radius: 8px;
    width: 100%;
    cursor: pointer; 
  }
  
  .v-data-table tbody tr:hover {
    background-color: #aaa8a8; 
    color: black;
  }
</style>
  