<script lang="ts">
    import axios from "axios";
    import type { Artist, Piece, Recording, Release } from "./types";
    import { ScrollArea } from "$lib/ui/ui/scroll-area";
    import { Separator } from "$lib/ui/ui/separator";
    import ReleaseArt from "./ReleaseArt.svelte";
    import PieceArt from "./PieceArt.svelte";
    import RecordingArt from "./RecordingArt.svelte";
    import ArtistArt from "./ArtistArt.svelte";

    // get token from parent
    export let token: string | null;

    // export the search query prop
    export let searchQuery: string = "";

    // setup axios instance
    const api = axios.create({
        baseURL: "http://localhost:9000",
    });

    // Define types for API responses
    type ApiArtist = {
        id: number;
        name: string;
        description: string | null;
        image_path: string | null;
    };

    type ApiRelease = {
        id: number;
        name: string;
        performer_ids: number[];
        description: string | null;
        image_path: string | null;
    };

    type ApiPiece = {
        id: number;
        name: string;
        movements: number | null;
        composer_ids: number[];
        songwriter_ids: number[];
        description: string | null;
    };

    type ApiRecording = {
        id: number;
        piece_id: number;
        release_id: number;
        performer_ids: number[];
        track_number: number;
        file_path: string;
    };

    // state for search results
    let performers: Artist[] = [];
    let composers: Artist[] = [];
    let songwriters: Artist[] = [];
    let releases: Release[] = [];
    let recordings: Recording[] = [];
    let pieces: Piece[] = [];

    // loading states with index signature
    let loading: { [key: string]: boolean } = {
        performers: false,
        composers: false,
        songwriters: false,
        releases: false,
        recordings: false,
        pieces: false,
    };

    // watch for changes in search query
    $: if (searchQuery) {
        performSearch();
    }

    async function performSearch() {
        performers = [];
        composers = [];
        songwriters = [];
        releases = [];
        recordings = [];
        pieces = [];

        Object.keys(loading).forEach((key) => {
            loading[key] = true;
        });

        try {
            // search performers
            const performerResponse = await api.post<{ message: ApiArtist[] }>(
                "/music/search/performer",
                { query: searchQuery, token },
            );
            performers = performerResponse.data.message.map((p: ApiArtist) => ({
                id: p.id,
                name: p.name,
                description: p.description,
                artistType: "performer",
                imagePath: p.image_path,
            }));

            // search composers
            const composerResponse = await api.post<{ message: ApiArtist[] }>(
                "/music/search/composer",
                { query: searchQuery, token },
            );
            composers = composerResponse.data.message.map((c: ApiArtist) => ({
                id: c.id,
                name: c.name,
                description: c.description,
                artistType: "composer",
                imagePath: c.image_path,
            }));

            // search songwriters
            const songwriterResponse = await api.post<{ message: ApiArtist[] }>(
                "/music/search/songwriter",
                { query: searchQuery, token },
            );
            songwriters = songwriterResponse.data.message.map(
                (s: ApiArtist) => ({
                    id: s.id,
                    name: s.name,
                    description: s.description,
                    artistType: "songwriter",
                    imagePath: s.image_path,
                }),
            );

            // search releases
            const releaseResponse = await api.post<{ message: ApiRelease[] }>(
                "/music/search/release",
                { query: searchQuery, token },
            );
            releases = await Promise.all(
                releaseResponse.data.message.map(async (r: ApiRelease) => {
                    const performers = await Promise.all(
                        r.performer_ids.map(async (id: number) => {
                            const performerResponse = await api.post<{
                                message: ApiArtist;
                            }>("/music/get/performer", { id });
                            const data = performerResponse.data.message;
                            return {
                                id: data.id,
                                name: data.name,
                                description: data.description,
                                artistType: "performer" as const,
                                imagePath: data.image_path,
                            };
                        }),
                    );

                    return {
                        id: r.id,
                        name: r.name,
                        performers,
                        description: r.description,
                        imagePath: r.image_path,
                    };
                }),
            );

            // search pieces
            const pieceResponse = await api.post<{ message: ApiPiece[] }>(
                "/music/search/piece",
                { query: searchQuery, token },
            );
            pieces = await Promise.all(
                pieceResponse.data.message.map(async (p: ApiPiece) => {
                    const composers = await Promise.all(
                        p.composer_ids.map(async (id: number) => {
                            const response = await api.post<{
                                message: ApiArtist;
                            }>("/music/get/composer", { id });
                            const data = response.data.message;
                            return {
                                id: data.id,
                                name: data.name,
                                description: data.description,
                                artistType: "composer" as const,
                                imagePath: data.image_path,
                            };
                        }),
                    );

                    const songwriters = await Promise.all(
                        p.songwriter_ids.map(async (id: number) => {
                            const response = await api.post<{
                                message: ApiArtist;
                            }>("/music/get/songwriter", { id });
                            const data = response.data.message;
                            return {
                                id: data.id,
                                name: data.name,
                                description: data.description,
                                artistType: "songwriter" as const,
                                imagePath: data.image_path,
                            };
                        }),
                    );

                    return {
                        id: p.id,
                        name: p.name,
                        movements: p.movements,
                        composers,
                        songwriters,
                        description: p.description,
                    };
                }),
            );

            // search recordings
            const recordingResponse = await api.post<{
                message: ApiRecording[];
            }>("/music/search/recording", { query: searchQuery, token });
            recordings = await Promise.all(
                recordingResponse.data.message.map(async (r: ApiRecording) => {
                    // get performers for the recording
                    const performers = await Promise.all(
                        r.performer_ids.map(async (id: number) => {
                            const performerResponse = await api.post<{
                                message: ApiArtist;
                            }>("/music/get/performer", { id });
                            const data = performerResponse.data.message;
                            return {
                                id: data.id,
                                name: data.name,
                                description: data.description,
                                artistType: "performer" as const,
                                imagePath: data.image_path,
                            };
                        }),
                    );

                    // get piece and its related composers and songwriters
                    const pieceResponse = await api.post<{ message: ApiPiece }>(
                        "/music/get/piece",
                        { id: r.piece_id },
                    );
                    const pieceData = pieceResponse.data.message;

                    // get composers for the piece
                    const composers = await Promise.all(
                        pieceData.composer_ids.map(async (id: number) => {
                            const composerResponse = await api.post<{
                                message: ApiArtist;
                            }>("/music/get/composer", { id });
                            const data = composerResponse.data.message;
                            return {
                                id: data.id,
                                name: data.name,
                                description: data.description,
                                artistType: "composer" as const,
                                imagePath: data.image_path,
                            };
                        }),
                    );

                    // get songwriters for the piece
                    const songwriters = await Promise.all(
                        (pieceData.songwriter_ids || []).map(
                            async (id: number) => {
                                const songwriterResponse = await api.post<{
                                    message: ApiArtist;
                                }>("/music/get/songwriter", { id });
                                const data = songwriterResponse.data.message;
                                return {
                                    id: data.id,
                                    name: data.name,
                                    description: data.description,
                                    artistType: "songwriter" as const,
                                    imagePath: data.image_path,
                                };
                            },
                        ),
                    );

                    // construct the complete piece object
                    const piece: Piece = {
                        id: pieceData.id,
                        name: pieceData.name,
                        movements: pieceData.movements,
                        composers,
                        songwriters,
                        description: pieceData.description,
                    };

                    // get release and its performers
                    const releaseResponse = await api.post<{
                        message: ApiRelease;
                    }>("/music/get/release", { id: r.release_id });
                    const releaseData = releaseResponse.data.message;

                    // get performers for the release
                    const releasePerformers = await Promise.all(
                        releaseData.performer_ids.map(async (id: number) => {
                            const performerResponse = await api.post<{
                                message: ApiArtist;
                            }>("/music/get/performer", { id });
                            const data = performerResponse.data.message;
                            return {
                                id: data.id,
                                name: data.name,
                                description: data.description,
                                artistType: "performer" as const,
                                imagePath: data.image_path,
                            };
                        }),
                    );

                    // construct the complete release object
                    const release: Release = {
                        id: releaseData.id,
                        name: releaseData.name,
                        performers: releasePerformers,
                        description: releaseData.description,
                        imagePath: releaseData.image_path,
                    };

                    // return the complete recording object
                    return {
                        id: r.id,
                        pieceName: pieceData.name,
                        piece,
                        release,
                        performers,
                        trackNumber: r.track_number,
                        filePath: r.file_path,
                    };
                }),
            );
        } catch (error) {
            console.error("Error performing search:", error);
        } finally {
            Object.keys(loading).forEach((key) => {
                loading[key] = false;
            });
        }
    }
</script>

{#if searchQuery}
    <div class="space-y-6">
        <!-- Releases Section -->
        {#if releases.length > 0}
            <div>
                <h2
                    class="text-2xl font-semibold tracking-tight text-slate-50 mb-4"
                >
                    Releases
                </h2>
                <ScrollArea orientation="both">
                    <div class="flex space-x-4 pb-4">
                        {#each releases as release}
                            <ReleaseArt
                                {release}
                                class="w-[250px]"
                                aspectRatio="portrait"
                                width={250}
                                height={330}
                            />
                        {/each}
                    </div>
                </ScrollArea>
            </div>
            <Separator class="my-4 bg-slate-800" />
        {/if}

        <!-- Pieces Section -->
        {#if pieces.length > 0}
            <div>
                <h2
                    class="text-2xl font-semibold tracking-tight text-slate-50 mb-4"
                >
                    Pieces
                </h2>
                <div class="grid gap-4">
                    {#each pieces as piece}
                        <PieceArt {piece} />
                    {/each}
                </div>
            </div>
            <Separator class="my-4 bg-slate-800" />
        {/if}

        <!-- Recordings Section -->
        {#if recordings.length > 0}
            <div>
                <h2
                    class="text-2xl font-semibold tracking-tight text-slate-50 mb-4"
                >
                    Recordings
                </h2>
                <div class="space-y-4">
                    {#each recordings as recording}
                        <RecordingArt {recording} />
                    {/each}
                </div>
            </div>
            <Separator class="my-4 bg-slate-800" />
        {/if}

        <!-- Artists Sections -->
        {#if performers.length > 0 || composers.length > 0 || songwriters.length > 0}
            <div>
                <h2
                    class="text-2xl font-semibold tracking-tight text-slate-50 mb-4"
                >
                    Artists
                </h2>

                {#if performers.length > 0}
                    <h3 class="text-lg font-medium text-slate-300 mb-2">
                        Performers
                    </h3>
                    <div class="grid grid-cols-2 md:grid-cols-3 gap-4 mb-4">
                        {#each performers as performer}
                            <ArtistArt artist={performer} />
                        {/each}
                    </div>
                {/if}

                {#if composers.length > 0}
                    <h3 class="text-lg font-medium text-slate-300 mb-2">
                        Composers
                    </h3>
                    <div class="grid grid-cols-2 md:grid-cols-3 gap-4 mb-4">
                        {#each composers as composer}
                            <ArtistArt artist={composer} />
                        {/each}
                    </div>
                {/if}

                {#if songwriters.length > 0}
                    <h3 class="text-lg font-medium text-slate-300 mb-2">
                        Songwriters
                    </h3>
                    <div class="grid grid-cols-2 md:grid-cols-3 gap-4">
                        {#each songwriters as songwriter}
                            <ArtistArt artist={songwriter} />
                        {/each}
                    </div>
                {/if}
            </div>
        {/if}

        <!-- Show loading state -->
        {#if Object.values(loading).some((v) => v)}
            <div class="text-center text-slate-400 py-8">Searching...</div>
        {/if}

        <!-- Show no results message -->
        {#if !Object.values(loading).some((v) => v) && performers.length === 0 && composers.length === 0 && songwriters.length === 0 && releases.length === 0 && pieces.length === 0}
            <div class="text-center text-slate-400 py-8">No results found</div>
        {/if}
    </div>
{/if}
