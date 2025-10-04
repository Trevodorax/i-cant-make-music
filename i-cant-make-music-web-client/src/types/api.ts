export interface MusicState {
  bpm: number;
  tracks: Track[];
  notes_per_beat: number;
}

export interface Track {
  id: string;
  sound: number[];
  notes: (number | null)[];
}

export interface CreateTrackRequest {
  sound_path: string;
  notes: (number | null)[];
}

export interface UpdateTrackRequest {
  sound_path?: string;
  notes?: (number | null)[];
}

export interface UpdateBpmRequest {
  bpm: number;
}

export interface GetStateResponse {
  state: MusicState;
}

export interface GetSoundsResponse {
  sounds: string[];
}

export interface CreateTrackResponse {
  created_track: Track;
  state: MusicState;
}

export interface UpdateTrackResponse {
  state: MusicState;
}

export interface DeleteTrackResponse {
  state: MusicState;
}

export interface ErrorResponse {
  message: string;
}