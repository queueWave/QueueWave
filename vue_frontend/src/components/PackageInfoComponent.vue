<template>
    <v-card>
      <v-card-title class="headline">Package Details</v-card-title>
      <v-card-text v-if="packageInfo">
        <p><strong>ID:</strong> {{ packageInfo.id }}</p>
        <p><strong>Received:</strong> {{ new Date(packageInfo.received).toLocaleString() }}</p>
        <p><strong>Size:</strong> {{ packageInfo.size }} bytes</p>
        <p><strong>Type:</strong> {{ packageInfo.message.type }}</p>
        <p><strong>Queue:</strong> {{ packageInfo.message.queue }}</p>
        <p><strong>Sender:</strong> {{ packageInfo.message.message.sender.sender }}</p>
        <p><strong>Token:</strong> {{ packageInfo.message.message.sender.token }}</p>
        <p><strong>Timestamp:</strong> {{ new Date(packageInfo.message.message.sender.timestamp).toLocaleString() }}</p>
        <p><strong>Data:</strong> {{ packageInfo.message.message.package.data }}</p>
      </v-card-text>
      <v-card-text v-else>
        Package not found.
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="blue darken-1" text @click="$emit('close')">Close</v-btn>
      </v-card-actions>
    </v-card>
</template>
  
<script>
  import axios from 'axios';
  
  export default {
    name: 'PackageInfoComponent',
    props: {
      id: {
        type: String,
        required: true,
      },
    },
    data() {
      return {
        packageInfo: null,
      };
    },
    async created() {
      await this.fetchPackage();
    },
    methods: {
      async fetchPackage() {
        try {
          const response = await axios.get(`http://localhost:8080/api/package/${this.id}`);
          this.packageInfo = response.data;
        } catch (error) {
          console.error('Error fetching package:', error);
          this.packageInfo = null;
        }
      },
    }
  };
</script>
  
<style scoped>
  .v-card {
    max-width: 600px;
    margin: auto;
  }
</style>
  