<script>
    import __wbg_init from "../../assets/game";
</script>

<main>
    <button
        on:click={async () => {
            await __wbg_init();
        }}>Start Game</button
    >
</main>
