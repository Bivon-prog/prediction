import axios from 'axios';

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';

export interface TeamStats {
  team_id: number;
  team_name: string;
  goals_scored_avg: number;
  goals_conceded_avg: number;
  attack_strength: number;
  defense_strength: number;
}

export interface Prediction {
  home_win_prob: number;
  draw_prob: number;
  away_win_prob: number;
  over_2_5_prob: number;
  confidence: number;
  is_premium: boolean;
}

export interface Match {
  id: string;
  fixture_id: number;
  home_team: TeamStats;
  away_team: TeamStats;
  match_date: string;
  league: string;
  prediction: Prediction | null;
}

export const fetchTodaysMatches = async (userId: string): Promise<Match[]> => {
  const response = await axios.get(`${API_BASE_URL}/matches/today`, {
    params: { user_id: userId },
  });
  return response.data;
};

export const initiatePayment = async (userId: string, phoneNumber: string) => {
  const response = await axios.post(`${API_BASE_URL}/pay/mpesa/initiate`, {
    user_id: userId,
    phone_number: phoneNumber,
  });
  return response.data;
};
