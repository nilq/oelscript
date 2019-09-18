<template>
  <div class="oeleditor" justify-center>
    <div style="text-align: center;">
      <img src="@/assets/logo.png" style="width: 75px">
      <br><br>
    </div>
    <v-card>
      <v-toolbar flat color="#dedede">
        <v-btn rounded style="width: 100px !important;" class="luksus-button" @click="compile">Compile</v-btn>
      </v-toolbar>
      <div id="editor" style="margin-top: -1px;">
        <textarea v-model="input" @input="update" @keydown="ignoreTab" ref="editor" class="oeleditor-editor" spellcheck="false" ></textarea>
        <div style="font-family: 'DejaVu Sans Mono'; padding: 15px; font-size: 14px;">
          <p>{{ output }}</p>
        </div>
      </div>
    </v-card>
  </div>
</template>

<script>
  const axios = require('axios');

  String.prototype.splice = function(idx, rem, str) {
    return this.slice(0, idx) + str + this.slice(idx + Math.abs(rem));
  }

  export default {
    name: 'Editor',
    data () {
      return {
        output: "Output will be shown here.",
        input: 'øl dict = {\n' +
                '  nice: "stick a finger in the soil"\n' +
                '  bad: "tuborg classic"\n' +
                '}\n' +
                '\n' +
                'øl main() =\n' +
                '  print("hello øl")\n' +
                '\n' +
                'øl fib(n) =\n' +
                '  øl n < 3:\n' +
                '    øl n\n' +
                '  ølse:\n' +
                '    øl fib(n) + fib(n - 1)',
      }
    },
    methods: {
      update () {},
      compile () {
        axios.post('http://localhost:8000/compile/', this.input, {
          headers: {
            'Access-Control-Allow-Origin': '*',
          }
        })
        .then((response) => {
          this.output = eval(response.data)
        })
      },
      ignoreTab(e) {
        if (e.keyCode === 9) {
          let idx = this.$refs.editor.selectionStart

          this.input = this.input.slice(0, idx) + '  ' + this.input.slice(idx);

          this.$refs.editor.setSelectionRange(idx + 2, idx + 2)

          e.preventDefault()
        }
      }
    }
  }
</script>

<style scoped>
  .luksus-button {
    border-radius: 10px !important;
    margin: auto;
  }
  .oeleditor {
    margin-top: 20px;
    margin-left: 12%;
    margin-right: 10%;
    width: 80% !important;
  }

  #editor {
    margin: 0;
    font-family: 'Helvetica Neue', Arial, sans-serif;
    color: #333;
    height: 400px;
  }

  .oeleditor-editor, #editor div {
    display: inline-block;
    width: 50%;
    height: 100%;
    vertical-align: top;
    box-sizing: border-box;
    padding: 0 20px;
  }

  .oeleditor-editor {
    border: none;
    border-right: 1px solid #ccc;
    resize: none;
    outline: none;
    background-color: #f6f6f6 !important;
    font-size: 14px;
    font-family: "DejaVu Sans Mono", courier, monospace;
    padding-top: 15px !important;
    overflow-y: scroll !important;
  }

  code {
    color: #f66;
  }
</style>
