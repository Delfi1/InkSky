<script setup lang="ts">
import { ref } from "vue"
import { invoke } from "@tauri-apps/api/core";

const name = ref('');
const email = ref('');
const password = ref('');
const password_check = ref('');

const error_msg = ref('');

async function debug(content: string) {
  await invoke("debug", {content: content})
}

async function error(content: string) {
  await invoke("error", {content: content});
}

async function is_email_taken(): Promise<boolean> {
  await debug("Check if email is taken...");
  return await invoke("is_email_taken", { email: email.value })
}

async function email_update() {
  let is_taken = await is_email_taken();

  if (is_taken) {
    await debug("Email is taken!!!");
    document.getElementById("Eml").style.borderColor = "#b61414";
    error_msg.value = 'This email is taken';
    (document.getElementById("reg_input") as HTMLInputElement).disabled = true;
  } else {
    await debug("Email is new...")
    document.getElementById("Eml").style.borderColor = "#C7BEBE";
    error_msg.value = '';
    (document.getElementById("reg_input") as HTMLInputElement).disabled = false;
  }
}

async function password_update() {
  if (password.value != password_check.value) {
    await error("The passwords don\'t match");

    document.getElementById("Pswd").style.borderColor = "#b61414";
    document.getElementById("PswdCheck").style.borderColor = "#b61414";
    (document.getElementById("reg_input") as HTMLInputElement).disabled = true;
    error_msg.value = 'The passwords don\'t match';
  } else {
    await debug("Passwords are match");

    document.getElementById("Pswd").style.borderColor = "#C7BEBE";
    document.getElementById("PswdCheck").style.borderColor = "#C7BEBE";
    (document.getElementById("reg_input") as HTMLInputElement).disabled = false;
    error_msg.value = '';
  }
}

async function register() {
  //(document.getElementById("reg_input") as HTMLInputElement).disabled = true;
   error_msg.value = await invoke("register", {name: name.value, email: email.value, password: password.value});
}
</script>

<template>
  <div class="wrapper">
    <h2>Registration</h2>
    <form action="#" @submit.prevent="register">
      <div class="input-box">
        <input type="text" v-model="name" placeholder="Enter your name" required>
      </div>
      <div class="input-box">
        <input type="email" v-model="email" @input="email_update" id="Eml" placeholder="Enter your email" required>
      </div>
      <div class="input-box">
        <input type="password" v-model="password" @input="password_update" id="Pswd" placeholder="Password" required>
      </div>
      <div class="input-box">
        <input type="password" v-model="password_check" @input="password_update" id="PswdCheck" placeholder="Confirm password" required>
      </div>
      <div class="error">
        <h3 v-text="error_msg"></h3>
      </div>
      <div class="input-box button" id="register-btn">
        <input type="submit" id="reg_input" value="Register">
      </div>
      <div class="text">
        <h3>Already have an account? <a href="#">Login</a></h3>
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
  width: 43px;
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
