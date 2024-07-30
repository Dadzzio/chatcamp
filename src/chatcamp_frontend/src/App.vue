<script lang="ts">
import { ref } from 'vue';
import { canisterId, chatcamp_backend, createActor } from '../../declarations/chatcamp_backend';
import { AuthClient } from '@dfinity/auth-client';
import { HttpAgent } from '@dfinity/agent';
import type { Identity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';

export default {
  data() {
    return {
      newchat: "",
      chats: [] as string[][],
      identity: undefined as undefined | Identity,
      principal : undefined as undefined | Principal,
      targetPrincipal: "",
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
            this.principal = identity.getPrincipal()
            console.log("Logged IN!", this.principal)
            this.identity = identity;
            await this.pobierzCzaty()
          }
        }
      )
      
      
      

      
    }
  }
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    {{ principal }} <button @click="login">login</button>
    <div>
      <input v-model="targetPrincipal"><button @click="pobierzCzaty">Pobierz Chat</button>
    </div>
    <div>
      <div v-for="chat in chats[0]">
        {{ chat }}
      </div>
    </div>
    <div>
      <textarea v-model="newchat"></textarea><button @click="dodajChatMSG">Dodaj notatke</button>
    </div>
  </main>
</template>