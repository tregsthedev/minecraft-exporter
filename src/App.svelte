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
      name = data.players.max
      let {version, players } = data



    } catch (error) {
      console.error('Error occurred:', error);
    }
    console.log(message);
  }

  fetchMessage();
</script>

<div class="header">
<h1 class="title"> Server: {serverName} </h1>

</div>

<h1>Hello world! {message}</h1>
<h3> <span class="server-name"> servername </span> </h3>
<p>
<span class="player-count"> player count: </span> {playerCount}
hi
</p>

<h2> hi {name} </h2>
