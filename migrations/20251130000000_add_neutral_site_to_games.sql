-- Add neutral_site column to games table
-- This field indicates whether a game was played at a neutral site
-- (not at either team's home venue)

ALTER TABLE games
ADD COLUMN neutral_site BOOLEAN DEFAULT FALSE;

-- Create index for filtering neutral site games
CREATE INDEX idx_games_neutral_site ON games (neutral_site);

-- Update comment
COMMENT ON COLUMN games.neutral_site IS 'TRUE if game played at neutral site, FALSE if played at home team venue';
