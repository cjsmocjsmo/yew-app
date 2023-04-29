// use yew_router::prelude::*;
use stylist::yew::use_style;
use yew::prelude::*;

#[function_component]
pub fn YTheLastManImgComp() -> Html {
    let b64_style = use_style!("
        padding: 1em;
        width: 250px;
        border-radius: 8px;
    ");

    html!(
        <>
        <img 
            class={b64_style} 
            src="data:image/avif;base64, AAAAHGZ0eXBhdmlmAAAAAGF2aWZtaWYxbWlhZgAAAOptZXRhAAAAAAAAACFoZGxyAAAAAAAAAABwaWN0AAAAAAAAAA
                AAAAAAAAAAAA5waXRtAAAAAAABAAAAImlsb2MAAAAAREAAAQABAAAAAAEOAAEAAAAAAAAYFgAAACNpaW5mAAAAAAAB
                AAAAFWluZmUCAAAAAAEAAGF2MDEAAAAAamlwcnAAAABLaXBjbwAAABNjb2xybmNseAACAAIABoAAAAAMYXYxQ4EEDA
                AAAAAUaXNwZQAAAAAAAAJYAAAC7gAAABBwaXhpAAAAAAMICAgAAAAXaXBtYQAAAAAAAAABAAEEgYIDhAAAGB5tZGF0
                EgAKBxkmZXu1oQgyiDBEfwEED7RKlAUVACVqhoaARYKoTMMgs93gMVFS8moaxKgeoDClYFxl2dbQ14INehzWjNShws
                +Plnp8ceIU2NdOp7dfSjvxzR52VvqdChR6HcTiYKBf2MtDEu9/6yhz5A5SS6iBrXuwKsPiMD42Po6Nv5tBH+ZeOWGl
                RhuYJMArE+6Ff9FhvaIDkrkBwnsY3uFezbaTR9U+lQvVvtLp4L/46fLsj9bEhgNpTgGBoI5zVC3xKngq0uBpibWDIf
                fo/+nSTyeVLQkyfXgEsplY8P/usXmCj3bR+MLgAHZnSMTkNxpLJftjgnxL4JFMF8H/Elj37qxSSPrvjmaIc2/QlYuG
                j79xGcov+XEWlSTydJh0+K7PLWyLR4ce7Avt540NOuZfpk6t1c6oSYGGdzRjU+Q0hL5Wz3W1lt9NSRZT0Sx1rGCmad
                h/zQxAdL6MMd8ordQSGiycNNcyMo80xGrLtee8Mtc3vC+4RZotX/aiCjnvnPkAX1ZX4S9/ZxvI9OZYzAvUGn2TMYcM
                fh/YG26AO7QzGJ/mf0vm/zJFXwPVROhY3Q6DoaoArEdm1zv4p58SCzHh3psmBm2tt1d6YGq20r77D3W3F+95FZLUV1
                VRZXQISTJfJRU26LLcf6+J0TYTEfM3dd/yPo1ELCKamTgKDFvqel3mWePB1XnoArHAsDbITUJUGMHSDxuYMJycVuic
                Tl0E6Y/+ggNUJh5I+rD+OVEUJF6qjbkJBIhR4x+tuXWF5JTuk9xxiwLO4EpwzIzbk7072uaj6vXQhp1HS2o26dAfGg
                bCqwKu5ma3RSOWqvEGNdhSgw+H2VSMGKi/PMutLeBlkes6/wav5JRsaCjTwdbH/I9k3rCFybHYU6Lg4qvR37osEPDB
                RNtkiF8iJYwjAbmSMPFMWRmU0FFPuW2C/AcBOBgWhc40OdjQrDYH/WJESOEl/4d+IsZ2FMXN8rG1/GIOsVWpxzjYfP
                /6CcEXSGP4GeYMHlPgmNKK59WwtJI/sIqFCsMZK9SrRO1ZE8IS8jRw+RXqZ07qUCO80VuRc0tpiUC0A11TuIdGnRDS
                jeuNwCUGTU3DFQuvF4Hwk3wbRFkGKlxFR6d/F16XG6kCKtBBq96vkks02LT+Wc3QXo6jCHCFnU80mhrvvpHn/ku6qP
                WQK4g34+uNR6u/LIZgm0Rbf/g13E2StdPEfTq80ENaXCHB8mAmTlYy5yFLW3RLrnOXj5pMTNWnplZQYNhAihO5yWQk
                5VDqHZd7cjHEIf7kyPif29I05YB2zQvX2lM9JjOywu6JFfXrqIdqIdMGO4ycbkeXHBE5qq9GQJUil8aP97pBT4Tq4l
                g145DgU5rtVBP6U6V5eiiGcPg9iSveIkc6hu5U15e6xjherHOb8VDuX0qLYgx8vZxtd/Efg9BXCTJuQ8bEWNXHMwfb
                P3SK9w66KheOeotYJ8PHx8VAywt8MDS6iF5Cj7ysQQQW3gqVze7ctDH4/iOJfHnLvRgzgRSY0Ou4TzXziTcI4fjNeY
                DiwHlOYbJqP0f6SgKpYyPqPG7DS43jcGaglG0GgcMXU2gRCSOLGU7Cm2QyWWF20pPpAO+Har4yMbjV75DB9xbdKnA6
                hN4IeXSzL+hSTsjo6/FBIOShg1Kxlr72IkWegF2MG1o6RwgBEcXPT+R6LTjUjTTAyi8I8h4EbYnjOYHlwYaBiIAZzQ
                81IFTieY55uxXEN5SmAnGsSJPF72x9FWaU0ajYPeju7a+kvwRM/h+Yzy4weNvZIgyFxiivV+OxoIovvfpBmJos0HNT
                kft/qc9E8MaIcBWiHxkTKyOgSyPsski3tRtiqKENocyB6FZ0DzFHe53xe9cyo0OjZweFI0Rv8C5xA4nQUvv6tTzVMc
                fZRHDcpeX4oZbcL4oWc53hDsZjeMfMUwc33NpYL0NMqAcVR34WwUvL6VthaehvOlj5YdHLeXSMkfyBUWJ65M+DBBAB
                hleEb9UVXaOrpYB4Lvo/xLv1Un2VIG1H+ucc2ehZJLd9XFZwZqq0gF4ko2CWoGPnJ2e3IanGW5Gb9sLy5bP1JiFs28
                2v/Jd/22Zz5oZ5akECXx4TU035Ixqnigs7wCyRubqqohmZa6rsOzECoJEbO2Cqz8hA6Tw+BkG2OQI4ckS2fzBWZaoT
                Lz5zlOf6t6ZrIKwaqiL2YjeVumVIWXh64nXY49Lm50Qe4JkglJr3PK+wSUW2QrHuuQmoIu87hNjXuMAPubLQ32ALCi
                LilyPOEQwEeUHrmSD3jEqfJUHOBBMya9wZaceTZTltdS9u7ky3rfT5ODR1vatxGybIZON75Ckx1cnRe3vKOJqNM3z+
                ZVpi0oh6TwtTsKjnb9C8r34/hOh0M7NVMWFIql9nHYBg3UKw3L16PZMyb4h+J0KVJF93rELKhxj3fgZEUrbdsjLq7L
                cwqDODZwUjPV63HpBNiDAGzoXBhrtCQ9z27GyLoAy+jOsf5gjB/rKTcRRcuprYcnosUIiyiIf0jONVRzfJWqa+wHYo
                23SOvaxFCknEmutbbPq/Wa4BYiQbZmDVScK/8fzLECeJAl67P6MnqMlvPWQjkqettM/lXeAZJtByE++5R8skpBdpw8
                bSjUYvBcqC+1KaMhrmnhcLwyl0o/UYsy3/hXduE+va+N/WV5236Yjp/vElJMASAffcBbwJw8756QiO2HQjpPv4XzvY
                NXzLryRjLPT3reBT5W/3z2D2VlxDTCC4QW5HsAbps+C2iPyi7hwM7E1pXhazDiCtw3w/JI6PVWxd9CRNCIptUMEKoF
                jra9AXrzDmPufcyr0Gin1XPvzM5g9AcTkBm9jCEF556YZJWVs2DcbaJ0hhTLkSMho1UHZ3ys/QqnHqALOqwmUIHakO
                jNhTSdO4CR9RBBgeB4cmjrMg0shaHRooOF5NZFFPGWavSfT+NlZ11hIO7Mg/NJvYnsNJZCq2iWvK1PXxUXXPAnWbmK
                D9qYnhnUR/nKHAHpsVKq7edzLrdXk90e9q4lwhKH3daJ7Xo4jLVmGpkKL7KPWYojif3wKCe0Hhee05oGtGZ/YsUJYc
                rdRzzEKSqOQGhpvGGefyv7A4A/we4Ekq/3tnnyhUy3hA0ST4FMGYzFsOpgbhASIon+zUDm7O1lio1avqm1pAXL65wf
                +lF0lx5MQxdCVj+5S9Ob2KMZGcQj81khaQRHUV3Ib629M4fuIl/xbXhwCXbkp0IgJm5odLvS27iUwPDQYqmnjK4a87
                1ZtDGNpDpHOxmppsZutnDXvrOfBQ7dPOSWISSLesZnBkdhVWBZ+jXW56fpg1zzQcVpubyQ/920iBAc8JyNhP/Ep+ty
                NPrdgaX3TN3dbZc1tg25aSD1NpENkddTweIv+Bv54tyDY2gvcmKoTaLf+49HDsCdTo1fpuiv9ogjhbi89+hPiQzWmA
                Eot0wKaOFThSMUnXPtrZP53x1ed4e7wAts6Lpr1sV047mnitu9vY3z0fvvFOpo3rrjSfYpLvPluVUwbQqjqqm91myB
                99laHZZMa0vfRZ8dd2Ji3XM41KhFIkiktKuDj3BEx/LusnAP1nxXuHt0eT9cugfOsEWqGKISpWH5L+eYt/j+s3KKek
                K2C+d5uUB8kpSn+AxhJufHECOwSoaD7vdb4iV+9DFZ0ETzWQ4HSJM9CiFbvS/29r15fiEv80KSqAErudOCwYePfnBc
                x6niBXb0k0RT2QubItWBEtpmw3lshl6SMIqE/2LOyWb2pHZc5VGf15YZKRcALrPAA7zyj7kGikxF/6mPYGxKxCXKus
                mWquS+AYYsOeotF7NZnHUmcEPK+ja+Lpfe/C5lhvH4uBUPalaIxoHVax90HsKxo5uJJurd7HaOpDUtsHPub7pnVFk/
                +9S1DTba9LzZp62edePyuIXBVdKsP6oMk4A1eGss4UnbvSR6DyFIvZvVcGI8P+Mu1awHUem6oHD3hRFxBIHz446wQC
                l4Qh3JfQyghihDiiLb7xfApE4Q47a7DWmIPRz58oTr/wKEddYSfTeIkbP8w11PMkyguLv7iqxWl2nEctFBhdgV1dMZ
                mDUhVOM7ybeHiWt5o1TkvAfLBi3S/m+G1UmK2LodrJSvd0VWwsifHc1QeKZRfL2sCNVmFUhF/QwFJAj05Qc9W0dUDZ
                l8h7Ntl/ZkxYR14fth7vd56J5cp6LmG1iYbjbfv/eaalqdEIG5JWU465Cx7Ze3n8eWD3KFJOg1XuWetoAcoc0EA6FD
                q5OFYqJYjlGgxG5u+WFIkOeYJyJmfi+T2YXqes7toBiGuIUfrKge8Q0n/3J6bzos5Hjz0X2aZoNupuXenVfP+N5K1M
                VMqxHZY38i8C0J4L3Hb9c7fZZwIDdFlJghdYwfyblYb+vQwV6Z/GJ7QiVolB2LNq2tFqJh5M5f6YDnppFhtdpIr8fR
                YW8vQb1OrVF9jDx0vFUy7P45zWveSe3YowAE6SkqedkfhOMEjHqzwRoAhWyIbAz699rTchsi/4H7ke4PXV1YXlOaOh
                aQKHK9/+v0YVIB1YqUEgJQpPk7+QOrHoJs6do2SPa7BX4iwp1SesjIOuyvM5QBQpQFQQqeEtZWeZfSDPOAwV6eWMi/
                17Q5y0Y+BQYil8k/x6ni9ryWDKMBhbSXI2NFuplzuCLFzsGUIp8aMFrxnKl3oRDhleCHFFotEs14IPy2baLAm6zwrj
                HWq/lRL/jl33bs5Us+bQt/mWTFyvp1tE/zQI1qovxYYyKXBzsqVH3tnJi4RqAZGDdTbzyuJR7+mzYy1198xiXBo3pa
                EjwngHrZlwC/5xP94nUMWuz4YoXFcu1LvNrF1hBUt1BNkjhVs06f0oeLnC0lZbc9HDl32hHBLP5nh12a3vNHSpijZ9
                GCKYERicGRNOHkPffrlb9LoVTRG+J69ZW7fNP+M5c/skoikDlOQcoZx7Py0EYykFL//2+Qr6FfNKFereaYRLTKc3jb
                R6KPGDpj4v96tSVsv1YzSjPLdGheyFG8dbJats31niOAUKATWClpakCLKjg+k3wisZkrJGSXtzdhsChwASyGVjMN23
                YreoFYbADrTwlkomz2/qumyYPbTpYAFrdiKg09TofNcfJkV4n/sKbZUs+NDjdlFV4jE+7A2jAVi2bFFx89UsRFysqB
                USKxcWj8c5oQ7qByZJ+xMVAcC3HlDuGMbqer6ujhk8N74WEA6Wwxg0fw2JG0NjxF93ncO1uOh7+3J7KfQ1UfXELUTB
                B+HpX79iedwgP8KAnkn6ZoZSpi5xMddZFUrwOxT+7JRZoB2KhkqWkZsN0OThUzsVmhj/7g8lB7gJvZrdhVAnH4kzoq
                Sg21lnYVWRQx1A8MFICwC0X6jNnypkhbvPqr4JHrOSmXQ3RgMOqrr0M3Coomqwz9JXJ1sJkC3TtU1wWvo7no4nwCsJ
                9HnaS2tDrs9kSs08H9O8qJRl27eqMD79Uv+WV7Oy7a9zMmXXgzHzm4mRhHvHuOb4Geu9qpAlZlScKbaxish8pALcTQ
                xmtHxEkLSwVFhg+P4VVbJQ7514jy1M9H1pRmMqcJCsW8LhHG1YT++6Cs99iqC6KkO1XsmMTT1IKai8DX15QMmmzzt3
                r/bKRsHbcIe4FlPm18IoAys5qjl+epbNtwHWMISlfChacumeAaVn3zOvBrbITE4HoZL0trM+rwRwiuWCu41LRRDD5G
                gw21pXSHqC7YjRij6Ggeun1Ib7u04KH78H5GMoqutmCKE/t9hXKEhvoK3pB8FpjlTrJNQZKteqa+8znhM7VrARdG0O
                9GIw224YPivorPEmEZgYxw1G18HH1uK2CJeNQy+pHL7UyfigRTL7IwGr1Ai7gdSzbbP8GumrnV2Sk083AYwuhJwfPa
                sK8u3w2vx6X1W0WM+XWi9mC3V5N9JwmsGo5uN9g2L+/VHtpDxaQDzr9K91in74Uhl+ivjRpnfzyF0GhucO0Tb+uCkz
                7No5z2igFlYSSDgJEy8yJY8aig9/5TdP3ka2aw5B6bj6sx7lxKuN60rNwZNnRy3L54DK5MbOQeMJkXdNpFLc7YDo7B
                NYgsCq5EtwyH2394DKP1Gf76+N2eja86mkUxRHmht0J1ZjnXQ7Wh1WbHF1vo3U6t/nWqmr72xIlETlPRBjG77mNF2k
                18aSnZAfnZxdot7++rFJ0QmTdItpgPtGepLeYSOLKN/apX2tM938VpLWkZWqbnfME/XuIPvzCwMVhvxnbGo0G3WrJ+
                7CcDjiN5HMKdoS5Ms7oXn7u6KAiOjBpj5lzcD/iKr/3CipZWxu3ebqtVGblHND/yw0d+hJZXfMlmMIdmVwOcASHa6c
                BCKS58ZMRnRfBwiX6wORrFvbysORgolxv2F7AxDvFg6bQM4Xsy/vKvbLwFkUj0eBCu7B0RxDESbOReW50y/VCz2D3v
                KXauDxCQuJKcsoX9ybGj+4NCkXoL+uV+17iH48hrq7TUV5Kkvjp0K3sF7XT6Dpw9Sp91gxuSZoFccm1swq+WP3Buiu
                jz3bv+KDJG2iFSOEExwHB7i9qFJgdYmIAd24GT8XC288p3LWABe2Iayk7P/WO6I4P1xzhcyjxwnAckXsBc9eu0Yrd6
                3JDiFVWcjFnP7yz90LxmO/VwbHQfGmeh826wFDpsNgoCugPQP7Pv+FPIU5FA4+Qfjl2GqJjeJSDTVjjqeex/DRPBEU
                yKZlRc4dE3ynHd22U72kF2dI2ZLy1baPYTur2ezbPvsWU62Zs/FivVbO9Hdn3lfewZelcSoP023oKRZAXoi0ZUUnCw
                iOkErMMIymPxKLF/OpIZSiPZ57wHYvDE0XteiFs2HzPamDUoTNTsTBKEj6ayD+UB7m4306uK7eltk0dF5nu53otYiX
                9io5SBVNwjzXaIN/HMG9+iizvNWNYIRl51w8zLNG2/4hokwbIFlvb5RQjRJKLl9c3hpALMIIhk3U2x8Xpc2GCttQQp
                uR4rdlA7BIi1sghe6e87hnBvY+i21Nj9soRxS9gunKfH9YslmkNz2VJrCOyhGzhS7B6KaaiDwLcBsIi9NA8IqNJNCS
                Lh7VCG1HwYsXgcCkCNNzUQlREbAyS8EyFEJs9wmw1jdk4WYCgf6I3egfPVS6XXI/sIft9mEyNwtZAFJUcpXBh3VB4S
                aGAgs1c52RDGejAFn+/fg0tEbqEr6OJEIx7xq8pq7RNLb9sbw70hoD8+Cv1weGkqFuaEUV2f+lNgVTHNJ6gM4dDI7r
                e25FZ8GG7bwsVNyw/NwVbxhdaC/CdIQuscsRhYcpNbr9kjT2O8hK4wHToW7Ov2uL8QSyDd0B1P6gdzj6/EFJsMttuR
                Zk1qEHxI4Lnx2737HDZi9yyOVTs8xZPycJ2HZ5OfCrmT3jhN5fs2VEa+keiBIeqkg/bOvoGCONzv65T0kogAIse9ia
                lA4hVpL2AduNbZ5WG5piSk/rItaGh4m4b2FTYUMXq8aCgXIvyvY63smllzc3fuM6vSmym6h8iUBAPoAMKC2oex760G
                8mayUnA8EuD8Jo8PRl2SYITujIYoxqVMagVrSMmW1vMfssEnHrOlQ4uexlAzBUYc+pTHhtjrEMuJWvQUhl0Udtrx3I
                sveF7Pz/iE0HpcpLz/PMWBgHeatTvCxadjQzNKdH1SfiXWrvB3SawqBB+ydb9AeLNqMDcqWnQLLmYnhc9vx5cEIEJo
                Pw2xD7xC4IGNYby/3oGX3KoP/QGIRd31cx3qZlVgF3/sODOwRB2RQyMpUx9mhfBp9rebwxWMT6xX8aSkZ2+i3/eSG+
                kvRfTZBTgV1+rk27daoTaQ87XMF0oDfDZl31ZWLU9a7mMW3zKmjCf/DOu6W8Cg4wbhj0ziqg0nCfJ/HjZU1AautvCf
                QHso38P1tLYpMf1oWcH17XBy9XH6DeGFTMQutLCJfRIcJDnTByr5u9bRi9x6OZxI3UVu2daHRG7tFXzhhEY4k3FsAX
                hc0hq11OhV5gLLBKryi1GKnkG8oc5g9TFn1DlbxKkRP7Smk56Z9FDWHglLCijNoREjjbT4t8QOCoyFGWp7wogFLMs3
                IjjN8n959TRJ7EpJOXq2jzAkI5qa29fzpL5x09LKFDFR2fGXxyF42vqOEunLebVmYR8OIjWYXi2niuJC9CYucPGCFH
                sidX/vjgr+tFwOq41qJh5d98Eik+9KL5zFPSlwaXCPg1nblQw2UQ8EXW4DyxBNNVmAafAhHvtbZ0OL6hlBmaBKpeoW
                f7eVUustzV9GjLzrlTaBLIeWpz3Y2ESw==" 
            />
        </>
    )
}