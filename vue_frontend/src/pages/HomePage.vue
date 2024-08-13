<template>
  <v-container class="fill-height d-flex flex-column justify-center align-center">
    <v-card class="pa-5" max-width="400" elevation="10">
      <v-card-title class="text-h5 text-center">Login</v-card-title>
      <v-card-text>
        <v-form @submit.prevent="login" class="loginform">
          <v-text-field
            v-model="username"
            label="Username"
            prepend-icon="mdi-account"
            required
          ></v-text-field>
          <v-text-field
            v-model="password"
            label="Password"
            type="password"
            prepend-icon="mdi-lock"
            required
          ></v-text-field>
          <v-btn type="submit" color="primary" class="mt-4" block>Login</v-btn>
        </v-form>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script>
import axios from 'axios';

export default {
  name: 'HomePage',
  data() {
    return {
      username: '',
      password: ''
    };
  },
  methods: {
    async login() {
      try {
        const response = await axios.post('http://localhost:8080/api/user/login', {
          username: this.username,
          password: this.password
        });
        console.log('Login successful:', response.data);
        this.$router.push('/dashboard');
      } catch (error) {
        console.error('Login failed:', error);
        alert('Login failed. Please check your credentials and try again.');
      }
    }
  }
};
</script>

<style scoped>
.fill-height {
  min-height: 100vh;
}
.v-card {
  border-radius: 12px;
  width: 100%;
}
.loginform{
  width: auto;
}
</style>
