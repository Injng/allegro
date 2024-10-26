<script lang="ts">
    import type { Piece } from "$lib/types";

    export let piece: Piece;
</script>

<div class="bg-slate-800 p-4 rounded-lg">
    <h3 class="text-lg font-medium text-slate-50">
        {piece.name}
    </h3>
    <p class="text-sm text-slate-400">
        {piece.composers.map((c) => c.name).join(", ")}
    </p>
    {#if piece.description}
        <p class="text-sm text-slate-300 mt-2">
            {piece.description}
        </p>
    {/if}
</div>
