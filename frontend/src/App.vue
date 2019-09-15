<template>
  <v-app>
      <Editor/>
  </v-app>
</template>

<script>
import Editor from './components/Editor';

export default {
  name: 'App',
  components: {
    Editor,
  },
  data: () => ({
    //
  }),
};
</script>
