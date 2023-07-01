import React, { useState, useEffect } from 'react';
import mainlogoImage from './main-logo.png';
import infoImage from './info.png';
import loginImage from './login.png';
import tipjarImage from './tipjar.png';
import depositImage from './deposit.png';
import withdrawImage from './withdraw.png';
import allStatsImage from './all-stats.png';
import tippedImage from './tipped.png';
import tippersImage from './tippers.png';
import depositedImage from './deposited.png';
import withdrawnImage from './withdrawn.png';
import tipsGivenImage from './tips-given.png';
import tipsReceivedImage from './tips-received.png';
import myStatsImage from './my-stats.png';
import cassieImage from './cassie.png';
import logo from './logo.png';
import './App.css';
import { useEthers, useContractFunction } from '@usedapp/core';
import detectEthereumProvider from '@metamask/detect-provider';

const placeholderContractAddress = '0x...'; // Replace with actual contract address
const placeholderABI = []; // Replace with actual ABI

function App() {
  const { activateBrowserWallet, account, library } = useEthers();
  const { state: depositState, send: deposit } = useContractFunction({
    abi: placeholderABI,
    address: placeholderContractAddress,
    method: 'deposit',
    args: [], // Replace with the method arguments if any
  });
  const { state: withdrawState, send: withdraw } = useContractFunction({
    abi: placeholderABI,
    address: placeholderContractAddress,
    method: 'withdraw',
    args: [], // Replace with the method arguments if any
  });

  const [loading, setLoading] = useState(true);
  const [infoPopupOpen, setInfoPopupOpen] = useState(false);

  useEffect(() => {
    const timer = setTimeout(() => {
      setLoading(false);
    }, 2500);

    return () => clearTimeout(timer);
  }, []);

  const connectWallet = async () => {
    const provider = await detectEthereumProvider();

    if (provider) {
      try {
        await provider.request({ method: 'wallet_switchEthereumChain', params: [{ chainId: '1287' }] });
      } catch (switchError) {
        if (switchError.code === 4902) {
          try {
            await provider.request({
              method: 'wallet_addEthereumChain',
              params: [
                {
                  chainId: '1287',
                  chainName: 'Moonriver',
                  nativeCurrency: {
                    name: 'Moonriver',
                    symbol: 'MOVR',
                    decimals: 18,
                  },
                  rpcUrls: ['https://rpc.moonriver.moonbeam.network'],
                  blockExplorerUrls: ['https://blockscout.moonriver.moonbeam.network/'],
                },
              ],
            });
          } catch (addError) {
            console.error('Failed to add Moonriver network');
          }
        }
      }

      await activateBrowserWallet();
    }
  };

  return (
    <div className="App">
      {loading ? (
        <div className="loading-screen">
          <img src={logo} className="logo-rotate" alt="Logo" />
        </div>
      ) : (
        <>
          <header className="header">
            <img src={mainlogoImage} className="logo" alt="Logo" />
          </header>
          <div className="button-container">
            <img
              src={infoImage}
              alt="Info"
              className="info-button"
              onClick={() => setInfoPopupOpen(true)}
            />
            {infoPopupOpen && (
              <div className="popup-container">
                <div className="popup-content">
                  <h2>Project Description</h2>
                  <p>Add your project description here.</p>
                  <button onClick={() => setInfoPopupOpen(false)}>Close</button>
                </div>
              </div>
            )}
            <div className="button-container">
              <img
                src={loginImage}
                alt="Login"
                className="login-button"
                onClick={connectWallet}
              />
            </div>
          </div>
          <div className="tipjar-container">
            <img src={tipjarImage} alt="Tip Jar" />
          </div>
          <div className="button-container">
            <img
              src={depositImage}
              alt="Deposit"
              className="deposit-button"
              onClick={deposit}
              disabled={depositState.status === 'Mining'}
            />
            <img
              src={withdrawImage}
              alt="Withdraw"
              className="withdraw-button"
              onClick={withdraw}
              disabled={withdrawState.status === 'Mining'}
            />
          </div>
          <div className="statistics-container">
            <img src={allStatsImage} alt="All Stats" className="all-stats" />
            <div className="statistics-row">
              <img src={tippedImage} alt="Tipped" className="tipped-image" data-statistic="10" />
              <span className="statistic-number">10</span>
            </div>
            <div className="statistics-row">
              <img src={tippersImage} alt="Tippers" className="tippers-image" data-statistic="5" />
              <span className="statistic-number">5</span>
            </div>
            <div className="statistics-row">
              <img
                src={depositedImage}
                alt="Deposited"
                className="deposited-image"
                data-statistic="100"
              />
              <span className="statistic-number">100</span>
            </div>
            <div className="statistics-row">
              <img
                src={withdrawnImage}
                alt="Withdrawn"
                className="withdrawn-image"
                data-statistic="50"
              />
              <span className="statistic-number">50</span>
            </div>
          </div>
          <div className="my-stats-container">
            <img src={myStatsImage} alt="My Stats" className="my-stats" />
            <div className="statistics-row">
              <img
                src={tipsGivenImage}
                alt="Tips Given"
                className="tips-given-image"
                data-statistic="20"
              />
              <span className="statistic-number">20</span>
            </div>
            <div className="statistics-row">
              <img
                src={tipsReceivedImage}
                alt="Tips Received"
                className="tips-received-image"
                data-statistic="15"
              />
              <span className="statistic-number">15</span>
            </div>
            <div className="statistics-row">
              <img
                src={depositedImage}
                alt="Deposited"
                className="deposited-image"
                data-statistic="200"
              />
              <span className="statistic-number">200</span>
            </div>
            <div className="statistics-row">
              <img
                src={withdrawnImage}
                alt="Withdrawn"
                className="withdrawn-image"
                data-statistic="100"
              />
              <span className="statistic-number">100</span>
            </div>
          </div>
          <div className="casswashere">
            <img src={cassieImage} alt="Cassie" />
          </div>
        </>
      )}
    </div>
  );
}

export default App;
