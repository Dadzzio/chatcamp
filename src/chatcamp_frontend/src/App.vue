<script lang="ts">
import { ref } from 'vue';
import { chatcamp_backend } from '../../declarations/chatcamp_backend';

export default {
  data() {
    return {
      newNote: "",
      notes: [] as string[]
    }
  },
  methods: {
    async dodajNotatke() {
      await chatcamp_backend.add_note(this.newNote)
      await this.pobierzNotatki()
    },
    async pobierzNotatki() {
      this.notes = await chatcamp_backend.get_notes()
    }
  },
  mounted(){
    this.pobierzNotatki()
  }
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
      <div>
      {{ notes }}
    </div>
    <div>
      <textarea v-model="newNote"></textarea><button @click="dodajNotatke">Dodaj notatke</button>
    </div>
  </main>
</template>