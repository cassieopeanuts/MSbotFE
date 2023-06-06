import React, { useState, useEffect } from 'react';
import './App.css';
import logo from './logo.png';
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

function App() {
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const timer = setTimeout(() => {
      setLoading(false);
    }, 5000);

    return () => clearTimeout(timer);
  }, []);

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
            <img src={infoImage} alt="Info" className="info-button" />
            <img src={loginImage} alt="Login" className="login-button" />
          </div>
          <div className="tipjar-container">
            <img src={tipjarImage} alt="Tip Jar" />
          </div>
          <div className="button-container">
            <img src={depositImage} alt="Deposit" className="deposit-button" />
            <img src={withdrawImage} alt="Withdraw" className="withdraw-button" />
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
              <img src={depositedImage} alt="Deposited" className="deposited-image" data-statistic="100" />
              <span className="statistic-number">100</span>
            </div>
            <div className="statistics-row">
              <img src={withdrawnImage} alt="Withdrawn" className="withdrawn-image" data-statistic="50" />
              <span className="statistic-number">50</span>
            </div>
          </div>
          <div className="my-stats-container">
            <img src={myStatsImage} alt="My Stats" className="my-stats" />
            <div className="statistics-row">
              <img src={tipsGivenImage} alt="Tips Given" className="tips-given-image" data-statistic="20" />
              <span className="statistic-number">20</span>
            </div>
            <div className="statistics-row">
              <img src={tipsReceivedImage} alt="Tips Received" className="tips-received-image" data-statistic="15" />
              <span className="statistic-number">15</span>
            </div>
            <div className="statistics-row">
              <img src={depositedImage} alt="Deposited" className="deposited-image" data-statistic="200" />
              <span className="statistic-number">200</span>
            </div>
            <div className="statistics-row">
              <img src={withdrawnImage} alt="Withdrawn" className="withdrawn-image" data-statistic="100" />
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
