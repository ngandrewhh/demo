import os

import gym
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from gym import spaces
from stable_baselines3 import PPO
from stable_baselines3.common.evaluation import evaluate_policy


class CustomEnv(gym.Env):
    """Custom Environment that follows gym interface"""

    def __init__(self, states, rewards):
        super(CustomEnv, self).__init__()
        self.states = states
        self.rewards = rewards

        self._episode_reward = np.empty(shape=(0))
        self._tot_rewards = np.empty(shape=(0))

        self._episode = 0
        self._eval_mode = False

        self._action_table = {0: -1, 1: 0, 2: 1}
        # Define action and observation space
        # They must be gym.spaces objects
        # Example when using discrete actions:
        self.action_space = spaces.Discrete(3)
        # Example for using image as input:
        self.observation_space = \
            spaces.Box(low=np.repeat(0.5, 20), high=np.repeat(1.5, 20), dtype=np.uint8)

    def step(self, action):
        done = False
        reward = self._action_table[action] * self.rewards[self._n].item()
        self._episode_reward = np.append(self._episode_reward, reward)
        self._n += 1

        if self._n == len(self.states):
            self.render()

            done = True
            states = -1
        else:
            states = self.states[self._n]

        return states, reward, done, dict()

    def reset(self):
        self._n = 0
        self._episode_reward = np.empty(shape=(0))
        self._episode += 1
        # self._done = False
        return self.states[self._n]

    def set_eval_mode(self):
        self._eval_mode = True

    def render(self):
        self._tot_rewards = np.append(self._tot_rewards, self._episode_reward.sum())
        print(f"finishing episode #{self._episode}; episode reward: {self._episode_reward.sum()}")

    def close(self):
        pass


if __name__ == "__main__":
    # data preprocessing
    csvs_px_dir = os.listdir('data')
    csvs_px = [pd.read_csv(os.path.join('data', und_name), index_col=0, parse_dates=True) for und_name in csvs_px_dir]

    for und_name, und_price in zip(csvs_px_dir, csvs_px):
        und_price.columns = [und_name.split(".")[1]]

    und_prices = pd.concat(csvs_px, axis=1)
    hs_prices = und_prices[['HSI', 'HSCE']].pct_change().dropna()
    cs_prices = und_prices[['CSI500', 'CSI1000I']].pct_change().dropna()
    us_prices = und_prices[['SPX', 'NDX']].pct_change().dropna()

    t1 = lambda dat: (1 + dat.values).cumprod(axis=0)
    t2 = lambda dat: dat[:, 0] / dat[:, 1]
    t3 = lambda dat: dat / dat.shift(260).fillna(1)

    # cs_env
    dat = pd.DataFrame(t2(t1(cs_prices)), index=cs_prices.index)
    dat_train = pd.concat([dat.shift(i) for i in range(20, 0, -1)] + [dat.pct_change().shift(-1), ], axis=1).dropna()
    states, rewards = t3(dat_train.iloc[:, :20]).values, dat_train.iloc[:, -1:].values
    cs_env = CustomEnv(states, rewards)

    # us_env
    dat = pd.DataFrame(t2(t1(us_prices)), index=us_prices.index)
    dat_train = pd.concat([dat.shift(i) for i in range(20, 0, -1)] + [dat.pct_change().shift(-1), ], axis=1).dropna()
    states, rewards = t3(dat_train.iloc[:, :20]).values, dat_train.iloc[:, -1:].values
    us_env = CustomEnv(states, rewards)
    
    # hs_env
    dat = pd.DataFrame(t2(t1(hs_prices)), index=hs_prices.index)
    dat_train = pd.concat([dat.shift(i) for i in range(20, 0, -1)] + [dat.pct_change().shift(-1), ], axis=1).dropna()
    states, rewards = t3(dat_train.iloc[:, :20]).values, dat_train.iloc[:, -1:].values
    hs_env = CustomEnv(states, rewards)
    

    # init training
    print(" starting training ".center(60, '-'))
    model = PPO.load("pairtrading_v0", env=cs_env, verbose=1)
    # model = PPO("MlpPolicy", env, )
    model.learn(total_timesteps=3300)
    # model.save("pairtrading_v0")
    print(" finishing training ".center(60, '-'))

    # cs eval
    model = PPO.load("pairtrading_v0", env=cs_env, verbose=1)
    mean_reward, std_reward = evaluate_policy(model, model.get_env(), n_eval_episodes=1)

    # us eval
    model = PPO.load("pairtrading_v0", env=us_env, verbose=1)
    mean_reward, std_reward = evaluate_policy(model, model.get_env(), n_eval_episodes=1)

    # hs eval
    model = PPO.load("pairtrading_v0", env=hs_env, verbose=1)
    mean_reward, std_reward = evaluate_policy(model, model.get_env(), n_eval_episodes=1)

    # train results viz
    z = open('log.txt')
    v = z.readlines()
    pd.Series([float(i.split(" ")[-1]) for i in v[1:3101]]).plot()
    plt.show()
    print()