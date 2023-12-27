<script>
  let message = '';
  let playerCount = 2;
  let data = '';
  let serverName = ''

  import { invoke } from '@tauri-apps/api/tauri';

  async function fetchMessage() {
    try {
      message = await invoke('api')
      data = JSON.parse(message)
      serverName = data.version.name
      let {version, players } = data



    } catch (error) {
      console.error('Error occurred:', error);
    }
    console.log(message);
  }

  fetchMessage();
</script>

<div class="header">
<h1> Minecraft Server Stats </h1>
<h1 class="title"> Server Name: {serverName} </h1>
<center> <input>  </center>

</div>


<h3> <span class="server-name"> servername </span> </h3>
<p>
<span class="player-count"> player count: </span> {playerCount}
hi
</p>

<h2> hi {name} </h2>

<style>
h1 {
color: blue;
}
</style>