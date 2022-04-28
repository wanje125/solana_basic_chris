use anchor_lang::prelude::*;

declare_id!("3hCvy79pX9ULsfmvzo6fd4pjnyncJq5Z3XVTTZEC3SV4");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_gifs = 0;
    
    
    Ok(())
  }
   // The function now accepts a gif_link param from the user. We also reference the user from the Context, 근데 아래에서 왜 _likes를 string으로 받는지는 모르겠다.
   pub fn add_gif(ctx: Context<AddGif>, gif_link: String, _check: bool, show_tip: bool, _likes: String, _id: String) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

    let clock = Clock::get().unwrap(); //anchor_lang::prelude::*에 clock 함수가 있다. 그리고 get으로 clock 정보를 불러올 수 있다.
                                        //unwrap은 앞의 함수가 에러가 나면 panic을 발생시켜준다. https://rinthel.github.io/rust-lang-book-ko/ch09-02-recoverable-errors-with-result.html 참조
    let likes_string = _likes.to_string();  // `parse()` works with `&str` and `String`!
    let my_likes = likes_string.parse::<u64>().unwrap(); //_likes를 string으로 to string을 해야 parse로 u64로 변환할 수 있다. my_likes는 u64이다.
                                                        //unwrap은 앞의 함수가 에러가 나면 panic을 발생시켜준다. 
	// Build the struct.
    let item = ItemStruct {
      gif_link: gif_link.to_string(),
      user_address: *user.to_account_info().key,
      _check: _check, //이건 어디서 받지? ctx처럼 위에 매개변수로 받는다.
      _id: _id.to_string(), //이것도 어디서 받지? ctx처럼 위에 매개변수로 받는다.
      my_likes: my_likes,
      timestamp: clock.unix_timestamp,
      show_tip: show_tip, //이것도 어디서 받지? ctx처럼 위에 매개변수로 받는다.
      tip: 0,
    };
		
	// Add it to the gif_list vector.
    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())
  }
  
  pub fn upvote_gif(ctx: Context<UpvoteGif>, _upvote: bool, gif_id: u16) -> Result <()> {
     // 위에서 gif_id는 react에서 map함수에서 index로 받는다. 즉 gif의 pk값이다.

    if _upvote == true {   //tick(check)이 false인경우 upvote는 true -> check가 눌러져있지 않은상태에서 클릭하면 like가 늘어난다. 
      let item = &mut ctx.accounts.base_account.gif_list[usize::from(gif_id)]; //Converts u16 to usize losslessly, 해당 gif_list 묶음 찾기
      item.my_likes += 1;                                                     //like +1
      item._check = true;                                                     //check true
    } else {
        let item = &mut ctx.accounts.base_account.gif_list[usize::from(gif_id)];
        item.my_likes -= 1;     
        item._check = false;
      }
    
    Ok(())
}

pub fn send_sol(ctx: Context<SendSol>, amount: String, gif_id: u16) -> Result <()> {
       
  let base_account = &mut ctx.accounts.base_account;

  let tip_string = amount.to_string();  // `parse()` works with `&str` and `String`!
    let my_tip = tip_string.parse::<u64>().unwrap(); //u64로 변환

    if my_tip > 0 {
  
  let ix = anchor_lang::solana_program::system_instruction::transfer(
      &ctx.accounts.from.key(), // from Pubkey
      &ctx.accounts.to.key(), // to Pubkey
      my_tip.into(), // lamport, into()로 다시 string으로 변환, 왜 그런지는 모르겠다.
  );
        //invoke를 이용해서 instruction과 [account_infos]를 집어넣고 cpi program을 호출한다. 트랜잭션이 실행된다는 의미인것같다.
          anchor_lang::solana_program::program::invoke(
    &ix,
    &[
      ctx.accounts.from.to_account_info(), //Signer이나 system account같은걸 account_info 구조체로 변환해준다. 
      ctx.accounts.to.to_account_info(), // []안에는 account_info 구조체가 필요하다.
    ]
  );

  
      
      let item = &mut ctx.accounts.base_account.gif_list[usize::from(gif_id)];
      item.tip += 1;

}
  Ok(())
  
}

  // The function now accepts a gif_link param from the user. We also reference the user from the Context
  // map(item)에서 index를 매개변수로 받는다. 근데 toString으로 string으로 변환한다. 왜그런지는 모르겠다. _index라는값으로 받는다. 
  pub fn remove_gif(ctx: Context<RemoveGif>, _index: u16) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

    //let my_string = _index.to_string();  // `parse()` works with `&str` and `String`!
    //let my_int = my_string.parse::<u64>().unwrap();

    base_account.gif_list.remove(_index.try_into().unwrap()); //해당 index 값 제거하고 제거한 값 리턴, try_into로 string에서 uszie같은걸로 변환하는것 같다.
    base_account.total_gifs -= 1;
    
    Ok(())
  }
}


#[derive(Accounts)]
pub struct StartStuffOff<'info> {                 //init은 program에 해당 계정을 생성한다(정보 저장을 위해서, rent fee 필요.). base_account는 react에서 프로그램 id의 pubkey로 받아온다.
  #[account(init, payer = user, space = 9000)] //payer로 지정된 account가 계정 생성에 필요한 솔라나를 지급한다. 
  pub base_account: Account<'info, BaseAccount>, //Account 구조체는 account_info를 wrapping한다. 그래서 account_info와 다른 데이터를 합칠 수 있다.
  #[account(mut)]                               //https://docs.rs/anchor-lang/0.4.0/anchor_lang/prelude/struct.AccountInfo.html 참조
  pub user: Signer<'info>, // https://docs.rs/anchor-lang/latest/anchor_lang/accounts/signer/struct.Signer.html 참조 , user는 react에서 사용자의 wallet pubkey로 받아온다.
   //When creating an account with init, the payer needs to sign the transaction. Signer.info.is_signer == true 인지 체크한다.
  pub system_program: Program <'info, System>, //Checks: account_info.key == expected_program & account_info.executable == true , 해당 프로그램과 account 계정이 같은지 확인
}//react에 signers array도 추가해서 넣었다. []안에 base_account를 넣어야한다.(react)
//We have to add the base_account here because whenever an account gets created, it has to sign its creation transaction. 
//We don't have to add user even though we gave it the Signer type in the program because it is the program provider and therefore signs the transaction by default.

// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)] 
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct RemoveGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpvoteGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct SendSol<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub from: Signer<'info>,
  #[account(mut)]
  pub to: AccountInfo<'info>,
  pub system_program: Program <'info, System>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub _check: bool,
    pub _id: String,
    pub my_likes: u64,
    pub timestamp: i64,
    pub show_tip: bool,
    pub tip: u64,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
	// Attach a Vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>,
}
