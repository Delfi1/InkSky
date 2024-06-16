<script setup lang="ts">
import { ref } from "vue"
import { invoke } from "@tauri-apps/api/core";

const email = ref('');
const password = ref('');
const error_msg = ref('');

async function login() {
  await invoke("login", { email: email.value, password: password.value })
}
</script>

<template>
  <div class="wrapper">
    <h2>Login</h2>
    <form action="#" @submit.prevent="login">
      <div class="input-box">
        <input type="email" v-model="email" id="Eml" placeholder="Enter your email" required>
      </div>
      <div class="input-box">
        <input type="password" v-model="password" id="Pswd" placeholder="Password" required>
      </div>
      <div class="error">
        <h3 v-text="error_msg" id="err_field"></h3>
      </div>
      <div class="input-box button" id="register-btn">
        <input type="submit" id="reg_input" value="Login">
      </div>
      <div class="text">
        <h3>Don't have account? <a href="#">Register</a></h3>
      </div>
    </form>
  </div>
</template>

<style>
@import url('https://fonts.googleapis.com/css?family=Poppins:400,500,600,700&display=swap');
*{
  margin: 0;
  padding: 0;
  box-sizing: border-box;
  font-family: 'Poppins', sans-serif;
}

body{
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #4070f4;
}

.wrapper {
  position: relative;
  max-width: 430px;
  width: 100%;
  background: #fff;
  padding: 37px;
  border-radius: 6px;
  box-shadow: 0 5px 10px rgba(0, 0, 0, 0.2);
}
.wrapper h2 {
  position: relative;
  font-size: 22px;
  font-weight: 600;
  color: #333;
}
.wrapper h2::before {
  content: '';
  position: absolute;
  top: 0;
  border-radius: 12px;
  bottom: 0;
  height: 3px;
  width: 38px;
  background: #4070f4;
}

.wrapper form {
  min-width: 260px;
  margin-top: 30px;
}

.wrapper form .input-box {
  height: 48px;
  margin: 16px 0;
}

form .input-box input {
  height: 100%;
  width: 100%;
  outline: none;
  padding: 0 13px;
  font-size: 17px;
  font-weight: 400;
  border: 2px solid #C7BEBE;
  color: #333;
  border-bottom-width: 3px;
  border-radius: 6px;
  transition: all 0.56s ease;
}

.input-box input:focus {
  border-color: #4070f4;
}

.input-box.button input {
  color: #fff;
  letter-spacing: 1px;
  border: none;
  background: #4070f4;
}

form h3 {
  color: #707070;
  font-size: 14px;
  font-weight: 500;
  margin-left: 10px;
}

.input-box.button input:hover {
  background: #1a56fa;
  cursor: pointer;
}

form .text h3 {
  color: #333;
  width: 100%;
  text-align: justify;
}

form .text h3 a {
  text-decoration: none;
  color: #4070f4;
}

form .text h3 a:hover {
  color: #1a56fa;
}

form .error h3 {
  color: #b61414;
}

</style>