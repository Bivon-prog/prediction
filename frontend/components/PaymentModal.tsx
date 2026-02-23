'use client';

import { useState } from 'react';
import { initiatePayment } from '@/lib/api';

interface PaymentModalProps {
  userId: string;
  onClose: () => void;
  onSuccess: () => void;
}

export default function PaymentModal({ userId, onClose, onSuccess }: PaymentModalProps) {
  const [phoneNumber, setPhoneNumber] = useState('254');
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState('');

  const handlePayment = async () => {
    if (phoneNumber.length < 12) {
      setMessage('Please enter a valid phone number (254XXXXXXXXX)');
      return;
    }

    setLoading(true);
    setMessage('');

    try {
      const response = await initiatePayment(userId, phoneNumber);
      setMessage('✅ ' + response.message);
      setTimeout(() => {
        onSuccess();
        onClose();
      }, 3000);
    } catch (error: any) {
      setMessage('❌ Payment failed: ' + (error.response?.data?.error || error.message));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="fixed inset-0 bg-black/80 backdrop-blur-sm flex items-center justify-center z-50 p-4 overflow-y-auto">
      <div className="bg-white rounded-2xl border border-gray-200 p-6 max-w-md w-full shadow-2xl my-8">
        <div className="text-center mb-4">
          <div className="w-14 h-14 bg-gradient-to-br from-amber-500 to-orange-500 rounded-full flex items-center justify-center mx-auto mb-3 shadow-lg">
            <span className="text-3xl">👑</span>
          </div>
          <h2 className="text-2xl font-bold text-gray-900 mb-2">Unlock VIP Access</h2>
          <p className="text-sm text-gray-600">
            24-hour access to premium predictions (75%+ confidence)
          </p>
        </div>

        <div className="bg-gradient-to-br from-blue-50 to-blue-100 rounded-xl p-4 mb-4 border border-blue-200">
          <h3 className="font-bold text-gray-900 mb-3 text-sm">What You Get:</h3>
          <ul className="space-y-2 mb-3">
            <li className="flex items-start space-x-2">
              <span className="text-blue-600 mt-0.5 text-sm">✓</span>
              <span className="text-xs text-gray-700">Unlimited high-confidence predictions (75%+)</span>
            </li>
            <li className="flex items-start space-x-2">
              <span className="text-blue-600 mt-0.5 text-sm">✓</span>
              <span className="text-xs text-gray-700">Detailed statistical analysis</span>
            </li>
            <li className="flex items-start space-x-2">
              <span className="text-blue-600 mt-0.5 text-sm">✓</span>
              <span className="text-xs text-gray-700">Real-time updates all day</span>
            </li>
            <li className="flex items-start space-x-2">
              <span className="text-blue-600 mt-0.5 text-sm">✓</span>
              <span className="text-xs text-gray-700">24-hour full premium access</span>
            </li>
          </ul>
          <div className="border-t border-blue-200 pt-3 mt-3">
            <div className="flex items-center justify-between">
              <span className="text-base font-bold text-gray-900">Total Price</span>
              <span className="text-2xl font-bold text-blue-600">KES 100</span>
            </div>
            <p className="text-xs text-gray-600 text-center mt-1">Valid for 24 hours</p>
          </div>
        </div>

        <div className="mb-4">
          <label className="block text-sm font-medium text-gray-700 mb-2">
            M-Pesa Phone Number
          </label>
          <input
            type="tel"
            value={phoneNumber}
            onChange={(e) => setPhoneNumber(e.target.value)}
            placeholder="254712345678"
            className="w-full px-4 py-2.5 bg-white border border-gray-300 rounded-lg text-gray-900 focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition text-sm"
            disabled={loading}
          />
          <p className="text-xs text-gray-500 mt-1.5">
            Enter your Safaricom M-Pesa number
          </p>
        </div>

        {message && (
          <div className={`mb-4 p-3 rounded-lg text-sm ${message.startsWith('✅') ? 'bg-green-50 text-green-700 border border-green-200' : 'bg-red-50 text-red-700 border border-red-200'}`}>
            {message}
          </div>
        )}

        <div className="flex gap-3">
          <button
            onClick={handlePayment}
            disabled={loading}
            className="flex-1 bg-gradient-to-r from-blue-600 to-blue-700 text-white py-3 rounded-lg font-bold hover:from-blue-700 hover:to-blue-800 disabled:bg-gray-300 disabled:text-gray-500 transition shadow-md text-sm"
          >
            {loading ? 'Processing...' : 'Pay KES 100'}
          </button>
          <button
            onClick={onClose}
            disabled={loading}
            className="px-5 py-3 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50 transition text-sm font-medium"
          >
            Cancel
          </button>
        </div>

        <p className="text-center text-xs text-gray-500 mt-4">
          🔒 Secure payment via M-Pesa
        </p>
      </div>
    </div>
  );
}
