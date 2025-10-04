import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { api } from '../api/client';
import type { CreateTrackRequest, UpdateTrackRequest, UpdateBpmRequest } from '../types/api';

export function useAppState() {
  return useQuery({
    queryKey: ['state'],
    queryFn: api.getState,
  });
}

export function useSounds() {
  return useQuery({
    queryKey: ['sounds'],
    queryFn: api.getSounds,
  });
}

export function useUpdateBpm() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: UpdateBpmRequest) => api.updateBpm(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['state'] });
    },
  });
}

export function useCreateTrack() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: CreateTrackRequest) => api.createTrack(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['state'] });
    },
  });
}

export function useUpdateTrack() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateTrackRequest }) =>
      api.updateTrack(id, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['state'] });
    },
  });
}

export function useDeleteTrack() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => api.deleteTrack(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['state'] });
    },
  });
}