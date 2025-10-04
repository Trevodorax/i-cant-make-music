import type {
  GetStateResponse,
  GetSoundsResponse,
  UpdateBpmRequest,
  CreateTrackRequest,
  CreateTrackResponse,
  UpdateTrackRequest,
  UpdateTrackResponse,
  DeleteTrackResponse,
} from '../types/api';

const API_BASE_URL = 'http://localhost:8080';

async function fetchApi<T>(endpoint: string, options?: RequestInit): Promise<T> {
  const response = await fetch(`${API_BASE_URL}${endpoint}`, {
    headers: {
      'Content-Type': 'application/json',
      ...options?.headers,
    },
    ...options,
  });

  if (!response.ok) {
    const error = await response.json().catch(() => ({ message: 'Unknown error' }));
    throw new Error(error.message || `HTTP ${response.status}`);
  }

  return response.json();
}

export const api = {
  getState: () => fetchApi<GetStateResponse>('/state'),

  getSounds: () => fetchApi<GetSoundsResponse>('/sounds'),

  updateBpm: (data: UpdateBpmRequest) =>
    fetchApi<GetStateResponse>('/bpm', {
      method: 'PUT',
      body: JSON.stringify(data),
    }),

  createTrack: (data: CreateTrackRequest) =>
    fetchApi<CreateTrackResponse>('/tracks', {
      method: 'POST',
      body: JSON.stringify(data),
    }),

  updateTrack: (id: string, data: UpdateTrackRequest) =>
    fetchApi<UpdateTrackResponse>(`/tracks/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    }),

  deleteTrack: (id: string) =>
    fetchApi<DeleteTrackResponse>(`/tracks/${id}`, {
      method: 'DELETE',
    }),
};