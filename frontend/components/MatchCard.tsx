'use client';

import { Match } from '@/lib/api';
import { format } from 'date-fns';

interface MatchCardProps {
  match: Match;
  onUnlock: () => void;
}

export default function MatchCard({ match, onUnlock }: MatchCardProps) {
  const prediction = match.prediction;
  const isPremiumLocked = prediction?.is_premium && prediction.home_win_prob === 0;

  const getWinnerClass = (prob: number) => {
    if (prob >= 50) return 'text-blue-600 font-bold';
    if (prob >= 35) return 'text-amber-600 font-semibold';
    return 'text-gray-500';
  };

  return (
    <div className="bg-white rounded-xl border border-gray-200 overflow-hidden hover:shadow-lg transition">
      {/* Match Header */}
      <div className="bg-gradient-to-r from-gray-50 to-gray-100 px-6 py-3 flex items-center justify-between border-b border-gray-200">
        <div className="flex items-center space-x-3">
          <span className="text-gray-700 font-semibold text-sm">{match.league}</span>
        </div>
        <span className="text-gray-600 text-sm font-medium">
          {format(new Date(match.match_date), 'HH:mm')}
        </span>
      </div>

      {/* Teams */}
      <div className="p-6">
        <div className="grid grid-cols-3 gap-6 items-center mb-6">
          {/* Home Team */}
          <div className="text-right">
            <h3 className="text-xl font-bold text-gray-900 mb-1">{match.home_team.team_name}</h3>
            <p className="text-xs text-gray-500">
              Avg: {match.home_team.goals_scored_avg.toFixed(1)} goals per game
            </p>
          </div>

          {/* VS */}
          <div className="text-center">
            <div className="bg-gray-100 rounded-lg py-2 px-4 border border-gray-200">
              <span className="text-gray-600 font-bold text-sm">VS</span>
            </div>
          </div>

          {/* Away Team */}
          <div className="text-left">
            <h3 className="text-xl font-bold text-gray-900 mb-1">{match.away_team.team_name}</h3>
            <p className="text-xs text-gray-500">
              Avg: {match.away_team.goals_scored_avg.toFixed(1)} goals per game
            </p>
          </div>
        </div>

        {/* Predictions */}
        {prediction && (
          <div className="border-t border-gray-200 pt-6">
            {isPremiumLocked ? (
              <div className="relative">
                <div className="blur-sm select-none pointer-events-none">
                  <div className="grid grid-cols-4 gap-3">
                    <div className="bg-gray-50 rounded-lg p-4 text-center border border-gray-200">
                      <p className="text-xs text-gray-500 mb-1">Home Win</p>
                      <p className="text-2xl font-bold text-gray-900">XX%</p>
                    </div>
                    <div className="bg-gray-50 rounded-lg p-4 text-center border border-gray-200">
                      <p className="text-xs text-gray-500 mb-1">Draw</p>
                      <p className="text-2xl font-bold text-gray-900">XX%</p>
                    </div>
                    <div className="bg-gray-50 rounded-lg p-4 text-center border border-gray-200">
                      <p className="text-xs text-gray-500 mb-1">Away Win</p>
                      <p className="text-2xl font-bold text-gray-900">XX%</p>
                    </div>
                    <div className="bg-gray-50 rounded-lg p-4 text-center border border-gray-200">
                      <p className="text-xs text-gray-500 mb-1">Over 2.5</p>
                      <p className="text-2xl font-bold text-gray-900">XX%</p>
                    </div>
                  </div>
                </div>
                <div className="absolute inset-0 flex items-center justify-center">
                  <button
                    onClick={onUnlock}
                    className="bg-gradient-to-r from-amber-500 to-orange-500 text-white px-8 py-4 rounded-xl font-bold shadow-2xl hover:shadow-amber-500/50 transform hover:scale-105 transition flex items-center space-x-2"
                  >
                    <span className="text-2xl">🔒</span>
                    <div className="text-left">
                      <div className="text-sm">Premium Prediction</div>
                      <div className="text-xs opacity-90">Unlock for KES 100</div>
                    </div>
                  </button>
                </div>
              </div>
            ) : (
              <>
                <div className="grid grid-cols-4 gap-3 mb-4">
                  <div className="bg-gray-50 rounded-lg p-4 text-center hover:bg-gray-100 transition border border-gray-200">
                    <p className="text-xs text-gray-600 mb-2 font-medium">Home Win</p>
                    <p className={`text-3xl font-bold ${getWinnerClass(prediction.home_win_prob)}`}>
                      {prediction.home_win_prob}%
                    </p>
                  </div>
                  <div className="bg-gray-50 rounded-lg p-4 text-center hover:bg-gray-100 transition border border-gray-200">
                    <p className="text-xs text-gray-600 mb-2 font-medium">Draw</p>
                    <p className={`text-3xl font-bold ${getWinnerClass(prediction.draw_prob)}`}>
                      {prediction.draw_prob}%
                    </p>
                  </div>
                  <div className="bg-gray-50 rounded-lg p-4 text-center hover:bg-gray-100 transition border border-gray-200">
                    <p className="text-xs text-gray-600 mb-2 font-medium">Away Win</p>
                    <p className={`text-3xl font-bold ${getWinnerClass(prediction.away_win_prob)}`}>
                      {prediction.away_win_prob}%
                    </p>
                  </div>
                  <div className="bg-gray-50 rounded-lg p-4 text-center hover:bg-gray-100 transition border border-gray-200">
                    <p className="text-xs text-gray-600 mb-2 font-medium">Over 2.5</p>
                    <p className={`text-3xl font-bold ${getWinnerClass(prediction.over_2_5_prob)}`}>
                      {prediction.over_2_5_prob}%
                    </p>
                  </div>
                </div>

                {/* Confidence Badge */}
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-2">
                    <span className="text-gray-600 text-sm font-medium">Confidence Level:</span>
                    <div className="bg-blue-100 text-blue-700 px-3 py-1 rounded-full text-sm font-bold">
                      {prediction.confidence}%
                    </div>
                  </div>
                  <div className="text-xs text-gray-500">
                    Statistical Analysis
                  </div>
                </div>
              </>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
