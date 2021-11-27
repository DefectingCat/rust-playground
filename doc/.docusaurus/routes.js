
import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/__docusaurus/debug',
    component: ComponentCreator('/__docusaurus/debug','3d6'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/config',
    component: ComponentCreator('/__docusaurus/debug/config','914'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/content',
    component: ComponentCreator('/__docusaurus/debug/content','c28'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/globalData',
    component: ComponentCreator('/__docusaurus/debug/globalData','3cf'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/metadata',
    component: ComponentCreator('/__docusaurus/debug/metadata','31b'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/registry',
    component: ComponentCreator('/__docusaurus/debug/registry','0da'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/routes',
    component: ComponentCreator('/__docusaurus/debug/routes','244'),
    exact: true
  },
  {
    path: '/blog',
    component: ComponentCreator('/blog','378'),
    exact: true
  },
  {
    path: '/blog/archive',
    component: ComponentCreator('/blog/archive','f4c'),
    exact: true
  },
  {
    path: '/blog/welcome',
    component: ComponentCreator('/blog/welcome','42b'),
    exact: true
  },
  {
    path: '/markdown-page',
    component: ComponentCreator('/markdown-page','be1'),
    exact: true
  },
  {
    path: '/docs',
    component: ComponentCreator('/docs','b4b'),
    routes: [
      {
        path: '/docs/Day 01/函数',
        component: ComponentCreator('/docs/Day 01/函数','2a3'),
        exact: true,
        'sidebar': "tutorialSidebar"
      },
      {
        path: '/docs/Day 01/基本概念',
        component: ComponentCreator('/docs/Day 01/基本概念','22d'),
        exact: true,
        'sidebar': "tutorialSidebar"
      },
      {
        path: '/docs/Day 01/数据类型',
        component: ComponentCreator('/docs/Day 01/数据类型','8b9'),
        exact: true,
        'sidebar': "tutorialSidebar"
      },
      {
        path: '/docs/Day 02/所有权',
        component: ComponentCreator('/docs/Day 02/所有权','95f'),
        exact: true,
        'sidebar': "tutorialSidebar"
      },
      {
        path: '/docs/Day 02/流程控制',
        component: ComponentCreator('/docs/Day 02/流程控制','11c'),
        exact: true,
        'sidebar': "tutorialSidebar"
      },
      {
        path: '/docs/intro',
        component: ComponentCreator('/docs/intro','aed'),
        exact: true,
        'sidebar': "tutorialSidebar"
      }
    ]
  },
  {
    path: '/',
    component: ComponentCreator('/','f49'),
    exact: true
  },
  {
    path: '*',
    component: ComponentCreator('*')
  }
];
