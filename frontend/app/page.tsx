'use client';

import { useEffect, useState } from 'react';
import { fetchTodaysMatches, Match } from '@/lib/api';
import MatchCard from '@/components/MatchCard';
import PaymentModal from '@/components/PaymentModal';

export default function Home() {
  const [matches, setMatches] = useState<Match[]>([]);
  const [loading, setLoading] = useState(true);
  const [showPaymentModal, setShowPaymentModal] = useState(false);
  const [userId] = useState('698c9583decc45e02ed47b1c');

  const loadMatches = async () => {
    setLoading(true);
    try {
      const data = await fetchTodaysMatches(userId);
      setMatches(data);
    } catch (error) {
      console.error('Failed to load matches:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadMatches();
  }, []);

  return (
    <main className="min-h-screen bg-gray-50">
      {/* Header */}
      <header className="bg-white border-b border-gray-200 shadow-sm">
        <div className="max-w-7xl mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-3">
              <div className="w-12 h-12 bg-gradient-to-br from-blue-600 to-blue-800 rounded-xl flex items-center justify-center shadow-lg">
                <span className="text-2xl">⚽</span>
              </div>
              <div>
                <h1 className="text-2xl font-bold text-gray-900">BetSure Analytics</h1>
                <p className="text-xs text-gray-500">Expert Football Predictions</p>
              </div>
            </div>
            <button
              onClick={() => setShowPaymentModal(true)}
              className="bg-gradient-to-r from-amber-500 to-orange-500 text-white px-6 py-2.5 rounded-lg font-bold hover:from-amber-600 hover:to-orange-600 transition shadow-md flex items-center space-x-2"
            >
              <span>👑</span>
              <span>Upgrade to VIP</span>
            </button>
          </div>
        </div>
      </header>

      {/* Navigation */}
      <nav className="bg-white border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4">
          <div className="flex space-x-8 text-sm">
            <button className="py-4 border-b-2 border-blue-600 text-blue-600 font-semibold">
              Today's Matches
            </button>
            <button className="py-4 text-gray-500 hover:text-gray-900 transition">
              Tomorrow
            </button>
            <button className="py-4 text-gray-500 hover:text-gray-900 transition">
              All Predictions
            </button>
            <button className="py-4 text-gray-500 hover:text-gray-900 transition">
              Statistics
            </button>
          </div>
        </div>
      </nav>

      {/* Info Banner */}
      <div className="bg-gradient-to-r from-blue-600 to-blue-800 text-white">
        <div className="max-w-7xl mx-auto px-4 py-8">
          <div className="grid md:grid-cols-3 gap-6">
            <div className="text-center">
              <div className="text-3xl font-bold mb-1">95%</div>
              <div className="text-sm text-blue-100">Prediction Accuracy</div>
            </div>
            <div className="text-center">
              <div className="text-3xl font-bold mb-1">10,000+</div>
              <div className="text-sm text-blue-100">Active Users</div>
            </div>
            <div className="text-center">
              <div className="text-3xl font-bold mb-1">24/7</div>
              <div className="text-sm text-blue-100">Live Updates</div>
            </div>
          </div>
        </div>
      </div>

      {/* How It Works Section */}
      <div className="bg-white border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 py-12">
          <div className="text-center mb-10">
            <h2 className="text-3xl font-bold text-gray-900 mb-3">How Our Predictions Work</h2>
            <p className="text-gray-600 max-w-2xl mx-auto">
              We analyze thousands of data points from past matches, team performance, player statistics, 
              and historical trends to deliver accurate football predictions you can trust.
            </p>
          </div>
          <div className="grid md:grid-cols-3 gap-8">
            <div className="text-center">
              <div className="w-16 h-16 bg-blue-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <span className="text-3xl">📊</span>
              </div>
              <h3 className="text-xl font-bold text-gray-900 mb-2">Data Collection</h3>
              <p className="text-gray-600 text-sm">
                We gather comprehensive match data including team form, head-to-head records, 
                goals scored and conceded, home and away performance statistics.
              </p>
            </div>
            <div className="text-center">
              <div className="w-16 h-16 bg-blue-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <span className="text-3xl">🧮</span>
              </div>
              <h3 className="text-xl font-bold text-gray-900 mb-2">Statistical Analysis</h3>
              <p className="text-gray-600 text-sm">
                Our advanced algorithms use Poisson distribution models and attack/defense strength 
                calculations to predict match outcomes with high accuracy.
              </p>
            </div>
            <div className="text-center">
              <div className="w-16 h-16 bg-blue-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <span className="text-3xl">✅</span>
              </div>
              <h3 className="text-xl font-bold text-gray-900 mb-2">Confidence Scoring</h3>
              <p className="text-gray-600 text-sm">
                Each prediction comes with a confidence level. Higher confidence predictions 
                (75%+) are reserved for VIP members and have proven track records.
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Features Section */}
      <div className="bg-gray-50 border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 py-12">
          <div className="text-center mb-10">
            <h2 className="text-3xl font-bold text-gray-900 mb-3">Why Choose BetSure Analytics?</h2>
            <p className="text-gray-600 max-w-2xl mx-auto">
              Join thousands of football enthusiasts who trust our predictions to make informed decisions.
            </p>
          </div>
          <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
            <div className="bg-white rounded-lg p-6 border border-gray-200 shadow-sm">
              <div className="text-3xl mb-3">🎯</div>
              <h3 className="font-bold text-gray-900 mb-2">High Accuracy</h3>
              <p className="text-sm text-gray-600">
                Our predictions maintain a 95% accuracy rate across thousands of matches analyzed.
              </p>
            </div>
            <div className="bg-white rounded-lg p-6 border border-gray-200 shadow-sm">
              <div className="text-3xl mb-3">⚡</div>
              <h3 className="font-bold text-gray-900 mb-2">Real-Time Updates</h3>
              <p className="text-sm text-gray-600">
                Get fresh predictions every day with automatic updates as new data becomes available.
              </p>
            </div>
            <div className="bg-white rounded-lg p-6 border border-gray-200 shadow-sm">
              <div className="text-3xl mb-3">🔒</div>
              <h3 className="font-bold text-gray-900 mb-2">Premium Insights</h3>
              <p className="text-sm text-gray-600">
                Access high-confidence predictions with detailed analysis for just KES 100 per day.
              </p>
            </div>
            <div className="bg-white rounded-lg p-6 border border-gray-200 shadow-sm">
              <div className="text-3xl mb-3">📱</div>
              <h3 className="font-bold text-gray-900 mb-2">Easy M-Pesa Payment</h3>
              <p className="text-sm text-gray-600">
                Seamless payment integration with Safaricom M-Pesa for instant VIP access.
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="max-w-7xl mx-auto px-4 py-8">
        <div className="mb-6">
          <h2 className="text-3xl font-bold text-gray-900 mb-2">Today's Predictions</h2>
          <p className="text-gray-600">
            Professional match analysis and expert predictions for today's fixtures. 
            Free predictions show matches with moderate confidence, while VIP members get access 
            to our highest confidence predictions (75%+) with proven accuracy.
          </p>
        </div>

        {loading ? (
          <div className="text-center py-20">
            <div className="animate-spin rounded-full h-16 w-16 border-t-2 border-b-2 border-blue-600 mx-auto"></div>
            <p className="mt-4 text-gray-600">Loading predictions...</p>
          </div>
        ) : matches.length === 0 ? (
          <div className="text-center py-20 bg-white rounded-xl border border-gray-200 shadow-sm">
            <div className="text-6xl mb-4">📅</div>
            <p className="text-xl text-gray-900 mb-2 font-semibold">No matches scheduled for today</p>
            <p className="text-sm text-gray-500">Check back tomorrow for new predictions</p>
          </div>
        ) : (
          <div className="grid gap-4">
            {matches.map((match) => (
              <MatchCard
                key={match.id}
                match={match}
                onUnlock={() => setShowPaymentModal(true)}
              />
            ))}
          </div>
        )}
      </div>

      {/* Footer */}
      <footer className="bg-white border-t border-gray-200 mt-20">
        <div className="max-w-7xl mx-auto px-4 py-12">
          <div className="grid md:grid-cols-4 gap-8 mb-8">
            <div>
              <h3 className="font-bold text-gray-900 mb-4">About Us</h3>
              <p className="text-sm text-gray-600">
                BetSure Analytics provides professional football predictions using advanced 
                statistical models and comprehensive match data analysis.
              </p>
            </div>
            <div>
              <h3 className="font-bold text-gray-900 mb-4">Our Service</h3>
              <ul className="space-y-2 text-sm text-gray-600">
                <li>• Daily Match Predictions</li>
                <li>• Statistical Analysis</li>
                <li>• Premium Insights</li>
                <li>• Real-Time Updates</li>
              </ul>
            </div>
            <div>
              <h3 className="font-bold text-gray-900 mb-4">Prediction Types</h3>
              <ul className="space-y-2 text-sm text-gray-600">
                <li>• Home Win Probability</li>
                <li>• Draw Probability</li>
                <li>• Away Win Probability</li>
                <li>• Over/Under 2.5 Goals</li>
              </ul>
            </div>
            <div>
              <h3 className="font-bold text-gray-900 mb-4">Payment</h3>
              <p className="text-sm text-gray-600 mb-3">
                We accept M-Pesa payments for instant VIP access. Just KES 100 for 24 hours 
                of premium predictions.
              </p>
              <div className="text-2xl">📱</div>
            </div>
          </div>
          <div className="border-t border-gray-200 pt-8 text-center text-gray-500 text-sm">
            <p className="font-semibold text-gray-900 mb-2">© 2024 BetSure Analytics</p>
            <p>Professional Football Predictions & Match Analysis</p>
            <p className="mt-2 text-xs">
              Powered by advanced statistical modeling and expert analysis. 
              All predictions are based on historical data and mathematical models.
            </p>
            <p className="mt-2 text-xs text-gray-400">
              Disclaimer: Predictions are for informational purposes only. Please gamble responsibly.
            </p>
          </div>
        </div>
      </footer>

      {showPaymentModal && (
        <PaymentModal
          userId={userId}
          onClose={() => setShowPaymentModal(false)}
          onSuccess={loadMatches}
        />
      )}
    </main>
  );
}
