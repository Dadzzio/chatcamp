<script lang="ts">
import { ref, registerRuntimeCompiler } from 'vue';
import { canisterId, chatcamp_backend, createActor } from '../../declarations/chatcamp_backend';
import { AuthClient } from '@dfinity/auth-client';
import { HttpAgent } from '@dfinity/agent';
import type { Identity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import type { UserData } from '../../declarations/chatcamp_backend/chatcamp_backend.did';

export default {
  data() {
    return {
      newchat: "",
      chats: [] as string[][],
      identity: undefined as undefined | Identity,
      principal : undefined as undefined | Principal,
      targetPrincipal: "",
      userData: undefined as undefined | UserData,
      newNickname: "",
      allUsers: [] as [Principal, UserData][],
    }
  },
  methods: {
    isUserLogged() {
      if (!this.identity || !this.principal || this.principal === Principal.anonymous()){
        throw new Error("User is not logged in.")
      }
      return {
        identity: this.identity,
        principal : this.principal
      }
    },
    validateTargetPrincipal() {
      const cleanTargetPrincipal = this.targetPrincipal.trim()
      if (this.targetPrincipal === ""){
        throw new Error("Wrong target.")
      }
      const targetPrincipal = Principal.fromText(cleanTargetPrincipal)
      if (!targetPrincipal || targetPrincipal === Principal.anonymous()){
        throw new Error("Wrong target.")
      }
      return targetPrincipal

    },
    getAuthClient(){
      this.isUserLogged()
      return createActor(canisterId, {
        agentOptions : {
          identity: this.identity
        }
      })
    },
    async getUserData(){
      const {identity, principal} = this.isUserLogged()
      const maybeUserData = await chatcamp_backend.get_user(principal as Principal)
      if (maybeUserData.length === 0){
        this.userData = undefined
      } else {
        this.userData = maybeUserData[0]
      }
      console.log("UserData: ", this.userData)
    },
    async dodajChatMSG() { 
      this.isUserLogged()
      const targetPrincipal = this.validateTargetPrincipal()
      const backend = this.getAuthClient()
      await backend.add_chat_msg(this.newchat, targetPrincipal)
      await this.pobierzCzaty()
    },
    async pobierzCzaty() {
      const {identity, principal} = this.isUserLogged()
      const targetPrincipal = this.validateTargetPrincipal()
      const chatPath = [identity.getPrincipal(), targetPrincipal].sort()
      this.chats = await chatcamp_backend.get_chat(chatPath)
    },
    
    
    async login(){
      const authClient = await AuthClient.create();
      await authClient.login(
        {identityProvider: 'http://b77ix-eeaaa-aaaaa-qaada-cai.localhost:4943/',
          onSuccess: async () => {
            const identity = authClient.getIdentity();
            this.principal = identity.getPrincipal();
            const principal =  identity.getPrincipal();
            console.log("Logged IN!", this.principal)
            this.identity = identity;
        
            console.log("UserData: ", this.userData)
            await this.getUserData()
            await this.getAllUsers()
          }
        }
      )
    },
    async logout() {
      const authClient = await AuthClient.create();
      await authClient.logout()
      this.identity = undefined;
      this.principal = undefined;
      this.chats = [];
      this.userData = undefined; 
    },
    async registerNickname() {
      const trimedNickname = this.newNickname.trim()
      const backend = this.getAuthClient()
      await backend.register_user(trimedNickname)
      await this.getUserData()
      await this.getAllUsers()
    },
    async getAllUsers(){
      this.allUsers =  await chatcamp_backend.get_users()
    }
  }
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <button v-if="!principal" @click="login">login</button>
    <button v-if="principal" @click="logout">logout</button>
    <div v-if="principal && !userData">
      <input v-model="newNickname" placeholder="wpisz nick"> <button @click="registerNickname">Zarejestruj się</button>
    </div>
    <div v-if="principal && userData">
      <h1>Witaj, {{ userData.nickname }}</h1>
    <div v-if="allUsers">
      <select v-model="targetPrincipal" @change="pobierzCzaty">
        <option disabled value="">Please select one</option>
        <option v-for="[userPrincipal, userData] in allUsers" :value="userPrincipal.toText()">{{ userData.nickname }}</option>
      </select>
\
    </div>
    <div>
      <div v-for="chat in chats[0]">
        {{ chat }}
      </div>
    </div>
    <div>
      <textarea v-model="newchat" placeholder="Wiadomość"></textarea><button @click="dodajChatMSG">Wyślij wiadomosć</button>
    </div></div>
  </main>
</template>