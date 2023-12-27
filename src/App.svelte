<script>
    import { invoke } from '@tauri-apps/api/tauri';
  async function something() {
    return message = await invoke('api')
  }
</script>

<h1>Hello world! {message}</h1>


