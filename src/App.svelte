<script>
    import { invoke } from '@tauri-apps/api/tauri';
    export const message = invoke('api')
</script>

<h1>Hello world! {message}</h1>


