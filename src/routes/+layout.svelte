<script>
    // https://www.youtube.com/watch?v=m_d7K28CSXg&t=331s
    
    import Nav from "$components/nav.svelte";
    import "../app.css";
</script>

<main class="bg-zinc-800 w-screen h-screen">
    <Nav />
    <slot />
</main>
