<script lang="ts">
    import type { Recording } from "$lib/types";
    import ReleaseArt from "$lib/ReleaseArt.svelte";

    export let recording: Recording;
</script>

<div class="bg-slate-800 p-4 rounded-lg">
    <div class="flex items-start gap-4">
        <!-- Release artwork -->
        <div class="flex-shrink-0">
            <ReleaseArt
                release={recording.release}
                class="w-[100px]"
                aspectRatio="square"
                width={100}
                height={100}
            />
        </div>

        <!-- Recording information -->
        <div class="flex-grow">
            <div class="flex justify-between items-start">
                <div>
                    <h3 class="text-lg font-medium text-slate-50">
                        {recording.piece.name}
                    </h3>
                    <p class="text-sm text-slate-400">
                        from {recording.release.name}
                    </p>
                </div>
                <span class="text-sm text-slate-500">
                    Track {recording.trackNumber}
                </span>
            </div>

            <!-- Composers -->
            {#if recording.piece.composers.length > 0}
                <p class="text-sm text-slate-300 mt-2">
                    Composers: {recording.piece.composers
                        .map((c) => c.name)
                        .join(", ")}
                </p>
            {/if}

            <!-- Performers -->
            {#if recording.performers.length > 0}
                <p class="text-sm text-slate-300">
                    Performers: {recording.performers
                        .map((p) => p.name)
                        .join(", ")}
                </p>
            {/if}

            <!-- Songwriters if any -->
            {#if recording.piece.songwriters && recording.piece.songwriters.length > 0}
                <p class="text-sm text-slate-300">
                    Songwriters: {recording.piece.songwriters
                        .map((s) => s.name)
                        .join(", ")}
                </p>
            {/if}
        </div>
    </div>
</div>
