import { writable } from "svelte/store";
import type { Recording } from "$lib/types";

interface PlayerState {
  isPlaying: boolean;
  currentRecording: Recording | null;
  currentTime: number;
  duration: number;
}

const initialState: PlayerState = {
  isPlaying: false,
  currentRecording: null,
  currentTime: 0,
  duration: 0,
};

export const playerStore = writable<PlayerState>(initialState);
