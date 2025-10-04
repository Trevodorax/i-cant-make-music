import type { FC } from "react";
import { useAppState } from "../hooks/useMusic";

export const Track: FC = () => {
    const { data, isLoading } = useAppState();

    if (isLoading) return <div>Loading...</div>;

    return (
        <div>
            {data?.state.tracks.map((track) => (
                <div key={track.id}>
                    <div>Track ID: {track.id}</div>
                    <div>Sound: {track.sound.length} bytes</div>
                </div>
            ))}
        </div>
    );
}
