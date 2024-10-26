<script lang="ts">
    import type { Artist } from "$lib/types";

    export let artist: Artist;
</script>

<div class="bg-slate-800 p-4 rounded-lg">
    <h4 class="font-medium text-slate-50">
        {artist.name}
    </h4>
    {#if artist.description}
        <p class="text-sm text-slate-400 mt-1">
            {artist.description}
        </p>
    {/if}
</div>
