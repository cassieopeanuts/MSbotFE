import React, { useState, useEffect } from 'react';
import './App.css';
import { ethers } from 'ethers';
import detectEthereumProvider from '@metamask/detect-provider';
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
  const [infoPopupOpen, setInfoPopupOpen] = useState(false);
  const [ethAddress, setEthAddress] = useState(null);
  const [web3, setWeb3] = useState(null);
  const [contract, setContract] = useState(null);
  const [hasProvider, setHasProvider] = useState(null);
  const initialState = { accounts: [], chainId: "" };
  const [wallet, setWallet] = useState(initialState);
  const [isConnecting, setIsConnecting] = useState(false);
  const [error, setError] = useState(false);
  const [errorMessage, setErrorMessage] = useState("");

  useEffect(() => {
    const timer = setTimeout(() => {
      setLoading(false);
    }, 2500);

    return () => clearTimeout(timer);
  }, []);

function MyComponent() {
  const [signer, setSigner] = useState();

  async function connectWallet() {
    if (window.ethereum) {
      const provider = new ethers.providers.Web3Provider(window.ethereum);
      const signer = provider.getSigner();
      setSigner(signer);
    }
  }

  return (
    <button onClick={connectWallet}>Connect Wallet</button> 
  );
}

const deposit = async () => {
  const amount = ethers.utils.parseEther('0.0000000000000001');
  const depositData = contract.interface.encodeFunctionData("deposit", [amount]);  
  const tx = {
    to: contractAddress,
    data: depositData
  };
  await signer.sendTransaction(tx);
}

const withdraw = async () => {
  const amount = ethers.utils.parseEther('0.0000000000000001');
  const withdrawData = contract.interface.encodeFunctionData("withdraw", [amount]);  
  const tx = {
    to: contractAddress,
    data: withdrawData  
  };
  await signer.sendTransaction(tx);     
}  

// Function to get Discord user ID
const getDiscordUserId = async () => {
  const response = await fetch('https://tipjar-back.vercel.app/api/main', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      /* any necessary data for the backend request, such as a Discord auth code */
    })
  });

  const data = await response.json();

  return data.discord_id;  // or whatever field contains the Discord user ID in the response
};

// Function to save user data
const saveUserData = async () => {
  if (ethAddress) {
    const actualDiscordUserId = await getDiscordUserId();

    fetch('https://tipjar-back.vercel.app/api/main', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        discord_id: actualDiscordUserId, 
        ethereum_address: ethAddress
      })
    }).then(response => {
      // handle response
    }).catch(error => {
      // handle error
    });
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
                      onClick={saveUserData} // Call saveUserData when the login button is clicked
                    />
                  </div>
            
          </div>
          <div className="tipjar-container">
            <img src={tipjarImage} alt="Tip Jar" />
          </div>
          <div className="button-container">
            <img src={depositImage} alt="Deposit" className="deposit-button" onClick={deposit} />
            <img src={withdrawImage} alt="Withdraw" className="withdraw-button" onClick={withdraw} />
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